use super::Property;
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Position {
    Position,
    Top,
    Bottom,
    Left,
    Right,
}

impl Render for Position {
    fn render(&self) -> String {
        match self {
            Position::Position => "position".to_owned(),
            Position::Top => "top".to_owned(),
            Position::Bottom => "bottom".to_owned(),
            Position::Left => "left".to_owned(),
            Position::Right => "right".to_owned(),
        }
    }
}

impl From<Position> for Property {
    fn from(pos: Position) -> Property {
        Property::Position(pos)
    }
}

#[cfg(test)]
mod position_tests {

    use super::{Position, Property, Render};

    #[test]
    fn render_position() {
        let result = Position::Position.render();
        let expected = "position";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_top() {
        let result = Position::Top.render();
        let expected = "top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_bottom() {
        let result = Position::Bottom.render();
        let expected = "bottom";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_left() {
        let result = Position::Left.render();
        let expected = "left";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_right() {
        let result = Position::Right.render();
        let expected = "right";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Position as Into<Property>>::into(Position::Top).render();
        let expected = Position::Top.render();
        assert_eq!(result, expected)
    }
}
