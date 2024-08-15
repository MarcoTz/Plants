use super::Property;
use crate::render::Render;

#[derive(Clone)]
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
