use std::fmt::Display;

use floem::{
    style_class,
    views::{label, Decorators, Label},
};

style_class!(pub H1);
style_class!(pub H2);
style_class!(pub H3);
style_class!(pub H4);
style_class!(pub H5);
style_class!(pub H6);

pub fn h1<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H1).style(|s| s.font_size(24.0).margin(4))
}

pub fn h2<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H2).style(|s| s.font_size(22.0).margin(4))
}

pub fn h3<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H3).style(|s| s.font_size(20.0).margin(4))
}

pub fn h4<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H4).style(|s| s.font_size(18.0).margin(4))
}

pub fn h5<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H5).style(|s| s.font_size(16.0).margin(4))
}

pub fn h6<S>(f: impl Fn() -> S + 'static) -> Label
where
    S: Display + 'static,
{
    label(f).class(H6).style(|s| s.font_size(14.0).margin(4))
}
