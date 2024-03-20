#![allow(clippy::module_name_repetitions)]

use std::{path::PathBuf, sync::Arc, time::Duration};

use bytes::Bytes;
use crossbeam_channel::Sender;
use dashmap::{DashMap, DashSet};
use reqwest::Url;
use xxhash_rust::xxh3::Xxh3Builder;

#[derive(Clone)]
pub struct AsyncCache {
    map: Arc<DashMap<Url, Bytes, Xxh3Builder>>,
    placeholder: Option<Bytes>,
    fetching: Arc<DashSet<Url, Xxh3Builder>>,
}

impl Default for AsyncCache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct CacheConfig {
    pub placeholder: Option<Bytes>,
    pub cache_on_disk: Option<PathBuf>,
    pub alive_time: Option<Duration>,
}

impl AsyncCache {
    #[must_use]
    pub fn new() -> Self {
        AsyncCache {
            map: Arc::new(DashMap::with_hasher(Xxh3Builder::new())),
            placeholder: None,
            fetching: Arc::new(DashSet::with_hasher(Xxh3Builder::new())),
        }
    }

    #[must_use]
    pub fn with_config(config: CacheConfig) -> Self {
        AsyncCache {
            map: Arc::new(DashMap::with_hasher(Xxh3Builder::new())),
            placeholder: config.placeholder,
            fetching: Arc::new(DashSet::with_hasher(Xxh3Builder::new())),
        }
    }

    pub fn url(&self, sender: &Sender<Bytes>, url: &str) {
        if let Some(placeholder) = &self.placeholder {
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

    #[cfg(feature = "async-std")]
    fn fetch(&self, url: Url, sender: Sender<Bytes>) {
        use async_compat::CompatExt;

        let shared_map = Arc::clone(&self.map);
        let shared_fetchlist = Arc::clone(&self.fetching);

        async_std::task::spawn(async move {
            let do_the_thing = || async {
                let response = reqwest::get(url.clone()).compat().await?;
                let bytes = response.bytes().compat().await?;

                sender.send(bytes.clone()).unwrap();

                shared_map.insert(url.clone(), bytes);

                reqwest::Result::Ok(())
            };

            if let Err(e) = do_the_thing().compat().await {
                eprintln!("{e}");
            }

            let _ = shared_fetchlist.remove(&url);
        });
    }

    #[cfg(feature = "tokio")]
    fn fetch(&self, url: Url, sender: Sender<Bytes>) {
        let shared_map = Arc::clone(&self.map);
        let shared_fetchlist = Arc::clone(&self.fetching);

        tokio::spawn(async move {
            let do_the_thing = || async {
                let response = reqwest::get(url.clone()).await?;
                let bytes = response.bytes().await?;

                sender.send(bytes.clone()).unwrap();

                shared_map.insert(url.clone(), bytes);

                reqwest::Result::Ok(())
            };

            if let Err(e) = do_the_thing().await {
                eprintln!("{e}");
            }

            let _ = shared_fetchlist.remove(&url);
        });
    }

    #[cfg(feature = "smol")]
    fn fetch(&self, url: Url, sender: Sender<Bytes>) {
        use async_compat::CompatExt;

        let shared_map = Arc::clone(&self.map);
        let shared_fetchlist = Arc::clone(&self.fetching);

        smol::spawn(async move {
            let do_the_thing = || async {
                let response = reqwest::get(url.clone()).compat().await?;
                let bytes = response.bytes().compat().await?;

                sender.send(bytes.clone()).unwrap();

                shared_map.insert(url.clone(), bytes);

                reqwest::Result::Ok(())
            };

            if let Err(e) = do_the_thing().compat().await {
                eprintln!("{e}");
            }

            let _ = shared_fetchlist.remove(&url);
        })
        .detach();
    }

    #[cfg(feature = "thread")]
    fn fetch(&self, url: Url, sender: Sender<Bytes>) {
        let shared_map = Arc::clone(&self.map);
        let shared_fetchlist = Arc::clone(&self.fetching);

        std::thread::spawn(move || {
            let do_the_thing = || {
                let response = reqwest::blocking::get(url.clone())?;
                let bytes = response.bytes()?;

                sender.send(bytes.clone()).unwrap();

                shared_map.insert(url.clone(), bytes);

                reqwest::Result::Ok(())
            };

            if let Err(e) = do_the_thing() {
                eprintln!("{e}");
            }

            let _ = shared_fetchlist.remove(&url);
        });
    }
}
