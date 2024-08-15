use super::Property;
use crate::render::Render;

#[derive(Clone)]
pub enum Size {
    Width,
    MaxWidth,
    Height,
    MaxHeight,
}

impl Render for Size {
    fn render(&self) -> String {
        match self {
            Size::Width => "width".to_owned(),
            Size::MaxWidth => "max-width".to_owned(),
            Size::Height => "height".to_owned(),
            Size::MaxHeight => "max-height".to_owned(),
        }
    }
}

impl From<Size> for Property {
    fn from(size: Size) -> Property {
        Property::Size(size)
    }
}
