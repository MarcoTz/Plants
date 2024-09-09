use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Border {
    Side(Direction),
    Color,
    Style,
    Radius,
    Collapse,
}

impl Render for Border {
    fn render(&self) -> String {
        match self {
            Border::Side(Direction::Top) => "border-top".to_owned(),
            Border::Side(Direction::Bottom) => "border-bottom".to_owned(),
            Border::Side(Direction::Left) => "border-left".to_owned(),
            Border::Side(Direction::Right) => "border-right".to_owned(),
            Border::Side(Direction::All) => "border".to_owned(),
            Border::Color => "border-color".to_owned(),
            Border::Style => "border-style".to_owned(),
            Border::Radius => "border-radius".to_owned(),
            Border::Collapse => "border-collapse".to_owned(),
        }
    }
}

impl From<Border> for Property {
    fn from(border: Border) -> Property {
        Property::Border(border)
    }
}

#[cfg(test)]
mod border_tests {
    use super::{Border, Direction, Property, Render};

    #[test]
    fn render_border_top() {
        let result = Border::Side(Direction::Top).render();
        let expected = "border-top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_bot() {
        let result = Border::Side(Direction::Bottom).render();
        let expected = "border-bottom";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_left() {
        let result = Border::Side(Direction::Left).render();
        let expected = "border-left";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_right() {
        let result = Border::Side(Direction::Right).render();
        let expected = "border-right";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_all() {
        let result = Border::Side(Direction::All).render();
        let expected = "border";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_color() {
        let result = Border::Color.render();
        let expected = "border-color";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_style() {
        let result = Border::Style.render();
        let expected = "border-style";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_radius() {
        let result = Border::Radius.render();
        let expected = "border-radius";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_collapse() {
        let result = Border::Collapse.render();
        let expected = "border-collapse";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_border_into() {
        let result = <Border as Into<Property>>::into(Border::Radius).render();
        let expected = Border::Radius.render();
        assert_eq!(result, expected);
    }
}
