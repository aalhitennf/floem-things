use floem::{
    id::Id,
    peniko::Color,
    style_class,
    unit::PxPctAuto,
    view::{View, ViewData},
    views::{empty, Decorators},
};

use crate::Orientation;

style_class!(pub SeparatorClass);

pub struct Separator {
    data: ViewData,

    size: f64,
    margin: f64,
    color: Color,

    orientation: Orientation,
}

impl Separator {
    #[must_use]
    pub fn new() -> Self {
        let id = Id::next();

        Separator {
            data: ViewData::new(id),
            size: 1.0,
            margin: 4.0,
            color: Color::rgb8(21, 22, 23),
            orientation: Orientation::Horizontal,
        }
    }

    #[must_use]
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn margin(mut self, margin: f64) -> Self {
        self.margin = margin;
        self
    }

    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Separator {
    fn view_data(&self) -> &ViewData {
        &self.data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.data
    }

    fn build(self) -> floem::view::AnyWidget {
        let size = self.size;
        let margin = self.margin;
        let color = self.color;
        let orientation = self.orientation;

        let view = match orientation {
            Orientation::Horizontal => separator_h(size, margin, color).any(),
            Orientation::Vertical => separator_v(size, margin, color).any(),
        }
        .class(SeparatorClass);

        view.build()
    }
}

fn separator_h(
    size: impl Into<PxPctAuto> + Copy + 'static,
    margin: impl Into<PxPctAuto> + Copy + 'static,
    color: impl Into<Color> + Copy + 'static,
) -> impl View {
    empty().style(move |s| {
        s.height(size)
            .margin_vert(margin)
            .width_full()
            .background(color.into())
    })
}

fn separator_v(
    size: impl Into<PxPctAuto> + Copy + 'static,
    margin: impl Into<PxPctAuto> + Copy + 'static,
    color: impl Into<Color> + Copy + 'static,
) -> impl View {
    empty().style(move |s| {
        s.width(size)
            .margin_horiz(margin)
            .height_full()
            .background(color.into())
    })
}
