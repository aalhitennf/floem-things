use floem::{
    event::{Event, EventListener},
    keyboard::{KeyCode, KeyEvent, PhysicalKey},
    peniko::Color,
    style::Style,
    unit::UnitExt,
    view::{AnyView, View},
    views::{container, text, Decorators},
};
use floem_things::{
    split::{Split, SplitDraggerHorizontalClass},
    Orientation,
};

fn main() {
    fn centered_textbox(txt: &str) -> AnyView {
        container(text(txt))
            .style(|s| s.justify_center().items_center().size_full())
            .any()
    }

    let top = centered_textbox("top");

    let btm_left = centered_textbox("btm_left");
    let btm_right = centered_textbox("btm_right");

    let btm = Split::new(btm_left, btm_right)
        .orientation(Orientation::Horizontal)
        .default_split(35.pct())
        .min_split(100.0)
        .dynamic(false)
        .any();

    let main_split = Split::new(top, btm)
        .orientation(Orientation::Vertical)
        .default_split(35.pct())
        .min_split(50.0)
        .style(|s| s.size_full());

    let view = container(main_split)
        .keyboard_navigatable()
        .style(Style::size_full);

    let id = view.id();

    let view = view
        .on_event_cont(EventListener::KeyDown, move |e| {
            if let Event::KeyDown(KeyEvent { key, .. }) = e {
                if !key.repeat && key.physical_key == PhysicalKey::Code(KeyCode::F11) {
                    id.inspect();
                }
            }
        })
        .style(|s| {
            s.class(SplitDraggerHorizontalClass, |s| {
                s.background(Color::REBECCA_PURPLE)
            })
        });

    floem::launch(|| view);
}
