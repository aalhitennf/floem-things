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
    buffer: Vec<u8>,
    fetch_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),

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
            buffer: vec![],
            fetch_channel,
            fetch_ok: false,
        }
    }

    pub fn placeholder(mut self, bytes: impl Into<Vec<u8>>) -> Self {
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

pub fn async_image(url: impl Into<String>, placeholder: Option<Vec<u8>>) -> impl View {
    AsyncImage::new(url).placeholder(placeholder.unwrap_or_default())
}

fn async_image_view(
    url: String,
    buffer: RwSignal<Vec<u8>>,
    tx: RwSignal<Sender<Vec<u8>>>,
    rx: Receiver<Vec<u8>>,
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
    });

    img(move || buffer.get().to_vec())
}

#[cfg(feature = "tokio")]
fn fetch_tokio(url: String, sender: Sender<Vec<u8>>, fetch_ok: RwSignal<bool>) {
    tokio::spawn(async move {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        if sender.send(bytes.to_vec()).is_ok() {
            fetch_ok.set(true);
        }

        std::result::Result::<(), reqwest::Error>::Ok(())
    });
}

#[cfg(feature = "async-std")]
fn fetch_async_std(url: String, sender: Sender<Vec<u8>>, fetch_ok: RwSignal<bool>) {
    async_std::task::spawn(async move {
        let mut response = surf::get(url).await?;
        let bytes = response.body_bytes().await?;
        if sender.send(bytes).is_ok() {
            fetch_ok.set(true);
        }

        std::result::Result::<(), surf::Error>::Ok(())
    });
}

#[cfg(feature = "smol")]
fn fetch_async_smol(url: String, sender: Sender<Vec<u8>>, fetch_ok: RwSignal<bool>) {
    use async_compat::CompatExt;

    smol::spawn(async move {
        let response = reqwest::get(url).compat().await?;
        let bytes = response.bytes().compat().await?;
        if sender.send(bytes.to_vec()).is_ok() {
            fetch_ok.set(true);
        }

        std::result::Result::<(), reqwest::Error>::Ok(())
    })
    .detach();
}
