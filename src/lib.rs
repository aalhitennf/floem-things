#[cfg(feature = "async-img")]
pub mod async_img;

pub mod h;
pub mod separator;
pub mod split;

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
