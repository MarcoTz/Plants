use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone)]
pub struct Padding {
    pub dir: Direction,
}

impl From<Padding> for Property {
    fn from(padding: Padding) -> Property {
        Property::Padding(padding)
    }
}

impl Render for Padding {
    fn render(&self) -> String {
        match self.dir {
            Direction::Top => "padding-top".to_owned(),
            Direction::Bottom => "padding-bottom".to_owned(),
            Direction::Left => "padding-left".to_owned(),
            Direction::Right => "padding-right".to_owned(),
            Direction::All => "padding".to_owned(),
        }
    }
}
