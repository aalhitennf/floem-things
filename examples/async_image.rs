use floem::{
    reactive::RwSignal,
    style::Style,
    view::View,
    views::{container, dyn_container, text, v_stack, Decorators},
    widgets::button,
};
use floem_things::async_img::async_image;

#[cfg(feature = "async-std")]
#[async_std::main]
async fn main() {
    floem::launch(|| create_view());
}

#[cfg(feature = "smol")]
fn main() {
    std::env::set_var("SMOL_THREADS", "4");

    smol::block_on(async {
        floem::launch(|| create_view());
    });
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    floem::launch(|| create_view());
}

#[cfg(feature = "thread")]
fn main() {
    floem::launch(|| create_view());
}

fn create_view() -> impl View {
    const URL: &str =
        "https://cdn.pixabay.com/photo/2016/12/20/15/49/eurasier-puppy-1920816_960_720.jpg";
    let placeholder = include_bytes!("../assets/placeholder.png");

    let show = RwSignal::new(false);

    let image = dyn_container(
        move || show.get(),
        move |show| {
            if show {
                async_image(URL)
                    .placeholder(placeholder.to_vec())
                    .style(Style::size_full)
                    .any()
            } else {
                container(text("Nothing here"))
                    .style(|s| s.justify_center().items_center())
                    .any()
            }
        },
    )
    .style(|s| s.size_full().justify_center().items_center());

    let button = button(|| "Hide/Show").on_click_stop(move |_| show.update(|s| *s = !*s));

    let view = container(v_stack((image, button)).style(Style::size_full)).style(|s| s.size_full());

    view
}
