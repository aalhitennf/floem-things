use floem::{
    event::{Event, EventListener},
    id::Id,
    kurbo::Size,
    peniko::Color,
    pointer::PointerMoveEvent,
    reactive::{with_scope, RwSignal, Scope},
    style::{CursorStyle, Style},
    style_class,
    unit::{Px, PxPct, PxPctAuto, UnitExt},
    view::{AnyView, View, ViewData},
    views::{container, empty, h_stack, v_stack, Decorators},
};

use crate::Orientation;

style_class!(pub SplitDraggerHorizontalClass);
style_class!(pub SplitDraggerVerticalClass);

pub struct Split {
    data: ViewData,
    cx: Scope,

    a: AnyView,
    b: AnyView,

    min_split: Px,
    default_split: PxPct,
    dynamic: bool,
    axis: Orientation,

    size: Size,
    split_value: PxPct,
    dragging: bool,

    dragger_size: Px,
    dragger_style: Style,
}

impl Split {
    #[must_use]
    pub fn new(a: AnyView, b: AnyView) -> Self {
        let id = Id::next();
        let cx = Scope::new();

        Self {
            data: ViewData::new(id),
            cx,

            a,
            b,

            min_split: 50.0.px(),
            default_split: PxPct::Pct(50.0),
            dynamic: true,
            axis: Orientation::Vertical,

            size: Size::ZERO,
            split_value: PxPct::Pct(50.0),
            dragging: false,

            dragger_size: 4.px(),
            dragger_style: Style::new(),
        }
    }

    /// Pixels
    #[must_use]
    pub fn min_split(mut self, value: impl Into<Px>) -> Self {
        self.min_split = value.into();
        self
    }

    /// Pixels or percent
    #[must_use]
    pub fn default_split(mut self, value: impl Into<PxPct> + Clone) -> Self {
        self.default_split = value.clone().into();
        self.split_value = value.into();
        self
    }

    /// Should split keep the ratio on resize if user has changed it
    #[must_use]
    pub fn dynamic(mut self, value: bool) -> Self {
        self.dynamic = value;
        self
    }

    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.axis = orientation;
        self
    }

    // Shortcut for dragger size
    #[must_use]
    pub fn dragger_size(mut self, size: impl Into<Px>) -> Self {
        self.dragger_size = size.into();
        self
    }

    /// Customize style
    #[must_use]
    pub fn dragger_style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.dragger_style = f(self.dragger_style);
        self
    }
}

fn build_dragger_style(dragging: RwSignal<bool>, size: Px, orientation: Orientation) -> Style {
    Style::new()
        .apply_if(matches!(orientation, Orientation::Horizontal), |s| {
            s.width(size).cursor(CursorStyle::ColResize)
        })
        .apply_if(matches!(orientation, Orientation::Vertical), |s| {
            s.height(size).cursor(CursorStyle::RowResize)
        })
        .background(Color::rgb8(205, 205, 205))
        .hover(|s| {
            s.apply_if(matches!(orientation, Orientation::Horizontal), |s| {
                s.width(size.0 + 2.0)
            })
            .apply_if(matches!(orientation, Orientation::Vertical), |s| {
                s.height(size.0 + 2.0)
            })
            .z_index(11)
            .background(Color::rgb8(41, 98, 218))
            .border_color(Color::rgb8(41, 98, 218))
        })
        .apply_if(dragging.get(), |s| {
            s.apply_if(matches!(orientation, Orientation::Horizontal), |s| {
                s.width(size.0 + 2.0)
            })
            .apply_if(matches!(orientation, Orientation::Vertical), |s| {
                s.height(size.0 + 2.0)
            })
            .z_index(100)
            .border_color(Color::rgb8(41, 98, 218))
            .background(Color::rgb8(41, 98, 218))
        })
}

impl View for Split {
    fn view_data(&self) -> &ViewData {
        &self.data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.data
    }

    fn build(self) -> floem::view::AnyWidget {
        let cx = self.cx;

        let min_split = self.min_split;
        let default_split = self.default_split;
        let dynamic = self.dynamic;

        let size = cx.create_rw_signal(self.size);
        let split_value = cx.create_rw_signal(self.split_value);
        let dragging = cx.create_rw_signal(self.dragging);

        let axis = self.axis;

        let a = self.a;
        let b = self.b;

        let dragger_size = self.dragger_size;

        let dragger_style =
            build_dragger_style(dragging, dragger_size, axis).apply(self.dragger_style);

        with_scope(cx, || match axis {
            Orientation::Horizontal => split_v(
                a,
                b,
                size,
                split_value,
                dragging,
                min_split,
                default_split,
                dynamic,
                dragger_style,
            )
            .any(),
            Orientation::Vertical => split_h(
                a,
                b,
                size,
                split_value,
                dragging,
                min_split,
                default_split,
                dynamic,
                dragger_style,
            )
            .any(),
        })
        .build()
    }
}

#[allow(clippy::too_many_arguments)]
#[inline]
fn split_v(
    a: impl View + 'static,
    b: impl View + 'static,
    size: RwSignal<Size>,
    width: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_split: Px,
    default_split: PxPct,
    dynamic: bool,
    dragger_style: Style,
) -> impl View {
    let dragger = dragger_v(
        width,
        dragging,
        min_split,
        default_split,
        size,
        dragger_style,
    );

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
        .on_event_stop(EventListener::DragOver, move |e| {
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

#[allow(clippy::too_many_arguments)]
#[inline]
fn split_h(
    a: impl View + 'static,
    b: impl View + 'static,
    size: RwSignal<Size>,
    height: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_split: Px,
    default_split: PxPct,
    dynamic: bool,
    dragger_style: Style,
) -> impl View {
    let dragger = dragger_h(
        height,
        dragging,
        min_split,
        default_split,
        size,
        dragger_style,
    );

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
        .on_event_stop(EventListener::DragOver, move |e| {
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

#[inline]
fn dragger_h(
    height: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_size: Px,
    default_split: PxPct,
    size: RwSignal<Size>,
    style: Style,
) -> impl View {
    empty()
        .class(SplitDraggerHorizontalClass)
        .style(move |s| {
            let size = size.get();
            let px = px_w(size.height, height.get());
            let max = size.height - min_size.0;

            let w = if px < min_size.0 {
                min_size.0
            } else if px > max {
                max
            } else {
                px
            };

            let w_pct = PxPctAuto::Pct(((w / size.height) * 100.0).abs());

            s.inset_top(w_pct)
                .absolute()
                .width_full()
                .z_index(10)
                .apply(style.clone())
        })
        .draggable()
        .dragging_style(|s| {
            s.background(Color::TRANSPARENT)
                .border(0)
                .cursor(CursorStyle::RowResize)
        })
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

#[inline]
fn dragger_v(
    width: RwSignal<PxPct>,
    dragging: RwSignal<bool>,
    min_size: Px,
    default_split: PxPct,
    size: RwSignal<Size>,
    style: Style,
) -> impl View {
    empty()
        .class(SplitDraggerVerticalClass)
        .style(move |s| {
            let size = size.get();
            let px = px_w(size.width, width.get());
            let max = size.width - min_size.0;

            let w = if px < min_size.0 {
                min_size.0
            } else if px > max {
                max
            } else {
                px
            };

            let w_pct = PxPctAuto::Pct(((w / size.width) * 100.0).abs());

            s.inset_left(w_pct)
                .absolute()
                .height_full()
                .z_index(10)
                .apply(style.clone())
        })
        .draggable()
        .dragging_style(|s| {
            s.background(Color::TRANSPARENT)
                .border(0)
                .cursor(CursorStyle::ColResize)
        })
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

#[inline]
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
