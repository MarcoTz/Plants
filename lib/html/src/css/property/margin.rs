use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone)]
pub struct Margin {
    pub dir: Direction,
}

impl From<Margin> for Property {
    fn from(margin: Margin) -> Property {
        Property::Margin(margin)
    }
}

impl Render for Margin {
    fn render(&self) -> String {
        match self.dir {
            Direction::Top => "margin-top".to_owned(),
            Direction::Bottom => "margin-bottom".to_owned(),
            Direction::Left => "margin-left".to_owned(),
            Direction::Right => "margin-right".to_owned(),
            Direction::All => "margin".to_owned(),
        }
    }
}
