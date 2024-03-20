use floem::{
    reactive::RwSignal,
    style::Style,
    view::View,
    views::{container, dyn_container, text, v_stack, Decorators},
    widgets::button,
};
use floem_things::async_img::async_image;

const URL: &str =
    "https://cdn.pixabay.com/photo/2016/12/20/15/49/eurasier-puppy-1920816_960_720.jpg";
const PLACEHOLDER: &[u8; 11211] = include_bytes!("../assets/placeholder.png");

#[cfg(feature = "async-std")]
#[async_std::main]
async fn main() {
    let show = RwSignal::new(true);

    let image = dyn_container(
        move || show.get(),
        move |show| {
            if show {
                async_image(URL, Some(PLACEHOLDER.to_vec()))
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

    floem::launch(|| view);
}

#[cfg(feature = "smol")]
fn main() {
    let show = RwSignal::new(true);

    let image = dyn_container(
        move || show.get(),
        move |show| {
            if show {
                async_image(URL, Some(PLACEHOLDER.to_vec()))
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

    smol::block_on(async {
        floem::launch(|| view);
    });
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    let show = RwSignal::new(true);

    let image = dyn_container(
        move || show.get(),
        move |show| {
            if show {
                async_image(URL, Some(PLACEHOLDER.to_vec()))
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

    floem::launch(|| view);
}
