use bytes::Bytes;
use crossbeam_channel::{Receiver, Sender};
use floem::{
    ext_event::create_signal_from_channel,
    id::Id,
    reactive::{create_effect, with_scope, RwSignal, Scope},
    view::{View, ViewData},
    views::img,
};

pub struct AsyncImage {
    data: ViewData,
    cx: Scope,

    url: String,
    buffer: Bytes,
    fetch_channel: (Sender<Bytes>, Receiver<Bytes>),

    fetch_ok: bool,
}

impl AsyncImage {
    pub fn new(url: impl Into<String>) -> Self {
        let id = Id::next();
        let cx = Scope::new();

        let url: String = url.into();
        let fetch_channel = crossbeam_channel::bounded(1);

        Self {
            data: ViewData::new(id),
            cx,
            url,
            buffer: Bytes::default(),
            fetch_channel,
            fetch_ok: false,
        }
    }

    pub fn placeholder(mut self, bytes: impl Into<Bytes>) -> Self {
        self.buffer = bytes.into();
        self
    }
}

impl View for AsyncImage {
    fn view_data(&self) -> &ViewData {
        &self.data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.data
    }

    fn build(self) -> floem::view::AnyWidget {
        let cx = self.cx;
        let url = self.url;

        let buffer = cx.create_rw_signal(self.buffer);

        let tx = cx.create_rw_signal(self.fetch_channel.0);
        let rx = self.fetch_channel.1;

        let fetch_ok = cx.create_rw_signal(self.fetch_ok);

        with_scope(cx, || async_image_view(url, buffer, tx, rx, fetch_ok)).build()
    }
}

pub fn async_image(url: impl Into<String>, placeholder: Option<impl Into<Bytes>>) -> impl View {
    AsyncImage::new(url).placeholder(placeholder.map_or(Bytes::default(), |p| p.into()))
}

fn async_image_view(
    url: String,
    buffer: RwSignal<Bytes>,
    tx: RwSignal<Sender<Bytes>>,
    rx: Receiver<Bytes>,
    fetch_ok: RwSignal<bool>,
) -> impl View {
    let image_signal = create_signal_from_channel(rx);

    let image_url = RwSignal::new(url);

    create_effect(move |_| {
        if let Some(v) = image_signal.get() {
            buffer.set(v);
        }
    });

    create_effect(move |_| {
        if fetch_ok.get() {
            return;
        }

        #[cfg(feature = "tokio")]
        fetch_tokio(image_url.get_untracked(), tx.get_untracked(), fetch_ok);

        #[cfg(feature = "async-std")]
        fetch_async_std(image_url.get_untracked(), tx.get_untracked(), fetch_ok);

        #[cfg(feature = "smol")]
        fetch_async_smol(image_url.get_untracked(), tx.get_untracked(), fetch_ok);

        #[cfg(feature = "thread")]
        fetch_thread(image_url.get_untracked(), tx.get_untracked(), fetch_ok);
    });

    img(move || buffer.get().to_vec())
}

#[cfg(feature = "tokio")]
fn fetch_tokio(url: String, sender: Sender<Bytes>, fetch_ok: RwSignal<bool>) {
    tokio::spawn(async move {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        if sender.send(bytes).is_ok() {
            fetch_ok.set(true);
        }

        std::result::Result::<(), reqwest::Error>::Ok(())
    });
}

#[cfg(feature = "async-std")]
fn fetch_async_std(url: String, sender: Sender<Bytes>, fetch_ok: RwSignal<bool>) {
    async_std::task::spawn(async move {
        if let Err(e) = fetch_compat(url, sender, fetch_ok).await {
            eprintln!("{e}");
        }
    });
}

#[cfg(feature = "smol")]
fn fetch_async_smol(url: String, sender: Sender<Bytes>, fetch_ok: RwSignal<bool>) {
    smol::spawn(async move {
        if let Err(e) = fetch_compat(url, sender, fetch_ok).await {
            eprintln!("{e}");
        }
    })
    .detach();
}

#[cfg(feature = "thread")]
fn fetch_thread(url: String, sender: Sender<Bytes>, fetch_ok: RwSignal<bool>) {
    let _ = std::thread::spawn(move || {
        let response = reqwest::blocking::get(url)?;
        let bytes = response.bytes()?;
        if sender.send(bytes).is_ok() {
            fetch_ok.set(true);
        }

        std::result::Result::<(), reqwest::Error>::Ok(())
    });
}

#[cfg(any(feature = "smol", feature = "async-std"))]
async fn fetch_compat(
    url: String,
    sender: Sender<Bytes>,
    fetch_ok: RwSignal<bool>,
) -> Result<(), reqwest::Error> {
    use async_compat::CompatExt;

    let response = reqwest::get(url).compat().await?;
    let bytes = response.bytes().compat().await?;
    if sender.send(bytes).is_ok() {
        fetch_ok.set(true);
    }

    Ok(())
}
