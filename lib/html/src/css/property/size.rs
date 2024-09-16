use super::Property;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod size_tests {

    use super::{Property, Render, Size};

    #[test]
    fn render_width() {
        let result = Size::Width.render();
        let expected = "width";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_maxwidth() {
        let result = Size::MaxWidth.render();
        let expected = "max-width";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_height() {
        let result = Size::Height.render();
        let expected = "height";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_maxheight() {
        let result = Size::MaxHeight.render();
        let expected = "max-height";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Size as Into<Property>>::into(Size::Height).render();
        let expected = Size::Height.render();
        assert_eq!(result, expected)
    }
}
