use floem::{
    reactive::{provide_context, RwSignal},
    style::Style,
    taffy::AlignContent,
    view::View,
    views::{container, dyn_container, h_stack_from_iter, text, v_stack, Decorators},
    widgets::button,
};
use floem_things::async_img::{
    async_image,
    cache::{AsyncCache, CacheConfig},
};

#[cfg(feature = "async-std")]
#[async_std::main]
async fn main() {
    floem::launch(main_view);
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    floem::launch(main_view);
}

#[cfg(feature = "smol")]
fn main() {
    std::env::set_var("SMOL_THREADS", "4");

    smol::block_on(async {
        floem::launch(main_view);
    });
}

#[cfg(feature = "thread")]
fn main() {
    floem::launch(main_view);
}

fn main_view() -> impl View {
    let show = RwSignal::new(false);

    let config = CacheConfig {
        placeholder: Some(include_bytes!("../assets/placeholder.png").to_vec().into()),
        ..Default::default()
    };

    let image_cache = AsyncCache::with_config(config);

    provide_context(image_cache);

    let urls: [&str; 3] = [
        "https://images.unsplash.com/photo-1601979031925-424e53b6caaa?q=80&w=300&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8M3x8cHVwcHl8ZW58MHx8MHx8fDA%3D",
        "https://plus.unsplash.com/premium_photo-1661338953443-f0757ecba8fd?q=80&w=300&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
        "https://images.unsplash.com/photo-1615751072497-5f5169febe17?q=80&w=300&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
    ];

    let view = dyn_container(
        move || show.get(),
        move |show| {
            if show {
                h_stack_from_iter(urls.into_iter().map(async_image).collect::<Vec<_>>())
                    .style(|s| {
                        s.size_full()
                            .justify_content(AlignContent::SpaceAround)
                            .items_center()
                    })
                    .any()
            } else {
                container(text(format!("Hiding\n{urls:#?}")))
                    .style(|s| s.size_full().justify_center().items_center())
                    .any()
            }
        },
    )
    .style(Style::size_full);

    let view_button =
        button(|| "Click to fetch/hide images").on_click_stop(move |_| show.update(|v| *v = !*v));

    let view_stack = v_stack((view, view_button)).style(Style::size_full);

    container(view_stack).style(|s| s.size_full().justify_center().items_center())
}
