use floem::{
    style_class,
    views::{label, Decorators},
};

style_class!(pub H1);
style_class!(pub H2);
style_class!(pub H3);
style_class!(pub H4);
style_class!(pub H5);
style_class!(pub H6);

pub fn h1<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H1).style(|s| s.font_size(24).margin(4))
}

pub fn h2<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H2).style(|s| s.font_size(22).margin(4))
}

pub fn h3<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H3).style(|s| s.font_size(20).margin(4))
}

pub fn h4<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H4).style(|s| s.font_size(18).margin(4))
}

pub fn h5<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H5).style(|s| s.font_size(16).margin(4))
}

pub fn h1<S>(f: impl Fn() -> S)
where
    S: Display,
{
    label(f).class(H6).style(|s| s.font_size(14).margin(4))
}
