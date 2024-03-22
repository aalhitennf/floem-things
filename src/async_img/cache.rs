#![allow(clippy::module_name_repetitions)]

use std::{path::PathBuf, sync::Arc, time::Duration};

use bytes::Bytes;
use crossbeam_channel::Sender;
use dashmap::{DashMap, DashSet};
use reqwest::Url;
use xxhash_rust::xxh3::{xxh3_64, Xxh3Builder};

#[derive(Clone)]
pub struct AsyncCache {
    map: Arc<DashMap<Url, Bytes, Xxh3Builder>>,
    config: CacheConfig,
    fetching: Arc<DashSet<Url, Xxh3Builder>>,
}

impl Default for AsyncCache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Clone)]
pub struct CacheConfig {
    pub placeholder: Option<Bytes>,
    pub local_cache_path: Option<PathBuf>,
    pub alive_time: Option<Duration>,
}

impl AsyncCache {
    #[must_use]
    pub fn new() -> Self {
        AsyncCache {
            map: Arc::new(DashMap::with_hasher(Xxh3Builder::new())),
            config: CacheConfig::default(),
            fetching: Arc::new(DashSet::with_hasher(Xxh3Builder::new())),
        }
    }

    #[must_use]
    pub fn with_config(mut config: CacheConfig) -> Self {
        if let Some(local_cache_path) = &config.local_cache_path {
            if let Err(e) = std::fs::create_dir_all(local_cache_path) {
                eprintln!("Cannot create local cache dir: {e}");
                config.local_cache_path = None;
            }
        }

        AsyncCache {
            map: Arc::new(DashMap::with_hasher(Xxh3Builder::new())),
            config,
            fetching: Arc::new(DashSet::with_hasher(Xxh3Builder::new())),
        }
    }

    pub fn url(&self, sender: &Sender<Bytes>, url: &str) {
        if let Some(placeholder) = &self.config.placeholder {
            if let Err(e) = sender.send(placeholder.clone()) {
                eprintln!("{e}");
            }
        }

        let Ok(url) = Url::parse(url) else {
            eprintln!("Invalid url: {url}");
            return;
        };

        if let Some(val) = self.map.get(&url) {
            if let Err(e) = sender.send(val.clone()) {
                eprintln!("{e}");
            }
            return;
        }

        if !self.fetching.contains(&url) {
            self.fetching.insert(url.clone());
            self.fetch(url, sender.clone());
        }
    }

    fn fetch(&self, url: Url, sender: Sender<Bytes>) {
        let shared_map = Arc::clone(&self.map);
        let shared_fetchlist = Arc::clone(&self.fetching);

        let local_file_path = self
            .config
            .local_cache_path
            .as_ref()
            .map(|p| p.join(xxh3_64(url.as_str().as_bytes()).to_string()));

        #[cfg(feature = "async-std")]
        async_std::task::spawn(async_fetch(
            url,
            local_file_path,
            shared_map,
            shared_fetchlist,
            sender,
        ));

        #[cfg(feature = "tokio")]
        tokio::spawn(async_fetch(
            url,
            local_file_path,
            shared_map,
            shared_fetchlist,
            sender,
        ));

        #[cfg(feature = "smol")]
        smol::spawn(async_fetch(
            url,
            local_file_path,
            shared_map,
            shared_fetchlist,
            sender,
        ))
        .detach();

        #[cfg(feature = "thread")]
        std::thread::spawn(move || {
            sync_fetch(
                &url,
                local_file_path,
                &shared_map,
                &shared_fetchlist,
                &sender,
            );
        });
    }
}

#[cfg(any(feature = "async-std", feature = "tokio", feature = "smol"))]
async fn async_fetch(
    url: Url,
    local_file_path: Option<PathBuf>,
    shared_map: Arc<DashMap<Url, Bytes, Xxh3Builder>>,
    shared_fetchlist: Arc<DashSet<Url, Xxh3Builder>>,
    sender: Sender<Bytes>,
) {
    use futures::{future::Either, pin_mut};

    let read_local = async { read_path(local_file_path.clone()).await.ok() };
    pin_mut!(read_local);

    let fetch_url = async { fetch(&url).await.ok() };
    pin_mut!(fetch_url);
    let (bytes, _) = futures::future::select(read_local, fetch_url)
        .await
        .factor_first();

    if let Some(bytes) = bytes {
        sender.send(bytes.clone()).unwrap();
        shared_map.insert(url.clone(), bytes.clone());
        let _ = shared_fetchlist.remove(&url);

        if let Some(local) = &local_file_path {
            let _ = write_bytes(local, bytes).await;
        }
    }
}

#[cfg(feature = "thread")]
fn sync_fetch(
    url: &Url,
    local_file_path: Option<PathBuf>,
    shared_map: &Arc<DashMap<Url, Bytes, Xxh3Builder>>,
    shared_fetchlist: &Arc<DashSet<Url, Xxh3Builder>>,
    sender: &Sender<Bytes>,
) {
    let handle_ok = |bytes: Bytes| {
        let _ = sender.send(bytes.clone());
        shared_map.insert(url.clone(), bytes);
        let _ = shared_fetchlist.remove(url);
    };

    // Try local
    if let Some(local_path) = &local_file_path {
        if let Ok(bytes) = std::fs::read(local_path) {
            handle_ok(bytes.into());
            return;
        }
    }

    // Fetch
    if let Ok(bytes) = fetch(url) {
        handle_ok(bytes.clone());

        if let Some(local_path) = local_file_path {
            let _ = write_bytes(&local_path, bytes);
        }
    }
}

#[cfg(any(feature = "async-std", feature = "smol"))]
async fn fetch(url: &Url) -> Result<Bytes, reqwest::Error> {
    use async_compat::CompatExt;

    let response = reqwest::get(url.clone()).compat().await?;
    let bytes = response.bytes().compat().await?;

    Ok(bytes)
}

#[cfg(feature = "tokio")]
async fn fetch(url: &Url) -> Result<Bytes, reqwest::Error> {
    let response = reqwest::get(url.clone()).await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

#[cfg(feature = "thread")]
fn fetch(url: &Url) -> Result<Bytes, reqwest::Error> {
    let response = reqwest::blocking::get(url.clone())?;
    let bytes = response.bytes()?;
    Ok(bytes)
}

#[cfg(any(feature = "async-std", feature = "tokio", feature = "smol"))]
async fn read_path(path: Option<PathBuf>) -> Result<Bytes, Box<dyn std::error::Error>> {
    let Some(path) = path else {
        futures::future::pending::<()>().await;
        return Ok(Bytes::default());
    };

    if !path.exists() {
        futures::future::pending::<()>().await;
        return Ok(Bytes::default());
    }

    #[cfg(feature = "smol")]
    let bytes = smol::fs::read(path).await?.into();

    #[cfg(feature = "async-std")]
    let bytes = async_std::fs::read(path).await?.into();

    #[cfg(feature = "tokio")]
    let bytes = tokio::fs::read(path).await?.into();

    Ok(bytes)
}

#[cfg(feature = "thread")]
fn read_path(path: Option<PathBuf>) -> Result<Bytes, std::io::Error> {
    let Some(path) = path else {
        return Ok(Bytes::default());
    };

    if !path.exists() {
        return Ok(Bytes::default());
    }

    let bytes = std::fs::read(path)?.into();
    Ok(bytes)
}

#[cfg(feature = "async-std")]
async fn write_bytes(path: &PathBuf, bytes: Bytes) -> Result<(), async_std::io::Error> {
    async_std::fs::write(path, bytes).await?;
    Ok(())
}

#[cfg(feature = "thread")]
fn write_bytes(path: &PathBuf, bytes: Bytes) -> Result<(), std::io::Error> {
    std::fs::write(path, bytes)?;
    Ok(())
}

#[cfg(feature = "smol")]
async fn write_bytes(path: &PathBuf, bytes: Bytes) -> Result<(), smol::io::Error> {
    smol::fs::write(path, bytes).await?;
    Ok(())
}

#[cfg(feature = "tokio")]
async fn write_bytes(path: &PathBuf, bytes: Bytes) -> Result<(), tokio::io::Error> {
    tokio::fs::write(path, bytes).await?;
    Ok(())
}
