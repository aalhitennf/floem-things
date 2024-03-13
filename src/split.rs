use floem::{
    event::{Event, EventListener},
    kurbo::Size,
    peniko::Color,
    pointer::PointerMoveEvent,
    reactive::RwSignal,
    style::Style,
    style_class,
    unit::{PxPct, PxPctAuto},
    view::View,
    views::{container, empty, h_stack, v_stack, Decorators},
};

style_class!(pub SplitDraggerHorizontalClass);
style_class!(pub SplitDraggerVerticalClass);

const fn to_auto(pct: PxPct) -> PxPctAuto {
    match pct {
        PxPct::Pct(p) => PxPctAuto::Pct(p),
        PxPct::Px(p) => PxPctAuto::Px(p),
    }
}

#[inline]
fn px_w(width: f64, w: PxPct) -> f64 {
    match w {
        PxPct::Pct(p) => width * (p / 100.0),
        PxPct::Px(p) => p,
    }
}

pub fn split_h(
    a: impl View + 'static,
    b: impl View + 'static,
    min_split: f64,
    default_split: PxPct,
    dynamic: bool,
) -> impl View {
    let size = RwSignal::new(Size::ZERO);
    let width = RwSignal::new(default_split);
    let dragging = RwSignal::new(false);

    let dragger = dragger_h(width, dragging, min_split, default_split, size);

    let a_con = container(a).style(move |s| s.min_width(min_split).width(to_auto(width.get())));
    let b_con = container(b).style(move |s| {
        let rs = size.get();
        let w = px_w(rs.width, width.get());
        let b_pct = ((w / rs.width) * 100.0).abs();
        let b_con_w = PxPctAuto::Pct(100.0 - b_pct);
        s.min_width(min_split).width(b_con_w)
    });

    h_stack((a_con, dragger, b_con))
        .style(Style::size_full)
        .on_resize(move |rect| {
            size.set(rect.size());
        })
        .on_event_cont(EventListener::DragOver, move |e| {
            if let Event::PointerMove(PointerMoveEvent { pos, .. }) = e {
                if dragging.get() {
                    if dynamic {
                        let pct = (pos.x / size.get().width) * 100.0;
                        width.set(PxPct::Pct(pct));
                    } else {
                        width.set(PxPct::Px(pos.x));
                    }
                }
            }
        })
        .style(Style::size_full)
}

pub fn split_v(
    a: impl View + 'static,
    b: impl View + 'static,
    min_split: f64,
    default_split: PxPct,
    dynamic: bool,
) -> impl View {
    let size = RwSignal::new(Size::ZERO);
    let height = RwSignal::new(default_split);
    let dragging = RwSignal::new(false);

    let dragger = dragger_v(height, dragging, min_split, default_split, size);

    let a_con = container(a).style(move |s| s.min_height(min_split).height(to_auto(height.get())));

    let b_con = container(b).style(move |s| {
        let rs = size.get();
        let w = px_w(rs.height, height.get());
        let b_pct = ((w / rs.height) * 100.0).abs();
        let b_con_w = PxPctAuto::Pct(100.0 - b_pct);
        s.min_height(min_split).height(b_con_w)
    });

    v_stack((a_con, dragger, b_con))
        .style(Style::size_full)
        .on_resize(move |rect| {
            size.set(rect.size());
        })
        .on_event_cont(EventListener::DragOver, move |e| {
            if let Event::PointerMove(PointerMoveEvent { pos, .. }) = e {
                if dragging.get() {
                    if dynamic {
                        let pct = (pos.y / size.get().height) * 100.0;
                        height.set(PxPct::Pct(pct));
                    } else {
                        height.set(PxPct::Px(pos.y));
                    }
                }
            }
        })
        .style(Style::size_full)
}

fn dragger_h(
    width: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_size: f64,
    default_split: PxPct,
    size: RwSignal<Size>,
) -> impl View {
    empty()
        .class(SplitDraggerHorizontalClass)
        .style(move |s| {
            let size = size.get();
            let px = px_w(size.width, width.get());
            let max = size.width - min_size;

            let w = if px < min_size {
                min_size
            } else if px > max {
                max
            } else {
                px
            };

            let w_pct = PxPctAuto::Pct(((w / size.width) * 100.0).abs());

            s.inset_left(w_pct)
                .apply_if(dragging.get(), |s| s.border_left(2))
        })
        .draggable()
        .dragging_style(|s| s.border_color(Color::TRANSPARENT))
        .on_event_stop(EventListener::DragStart, move |_| {
            dragging.set(true);
        })
        .on_event_stop(EventListener::DragEnd, move |_| {
            dragging.set(false);
        })
        .on_event_stop(EventListener::DoubleClick, move |_| {
            width.set(default_split);
            dragging.set(false);
        })
}

fn dragger_v(
    height: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_size: f64,
    default_split: PxPct,
    size: RwSignal<Size>,
) -> impl View {
    empty()
        .style(move |s| {
            let size = size.get();
            let px = px_w(size.height, height.get());
            let max = size.height - min_size;

            let w = if px < min_size {
                min_size
            } else if px > max {
                max
            } else {
                px
            };

            let w_pct = PxPctAuto::Pct(((w / size.height) * 100.0).abs());

            s.apply_if(dragging.get(), |s| s.border_top(2))
                .inset_top(w_pct)
        })
        .draggable()
        .dragging_style(|s| s.border_color(Color::TRANSPARENT))
        .on_event_stop(EventListener::DragStart, move |_| {
            dragging.set(true);
        })
        .on_event_stop(EventListener::DragEnd, move |_| {
            dragging.set(false);
        })
        .on_event_stop(EventListener::DoubleClick, move |_| {
            height.set(default_split);
            dragging.set(false);
        })
}
