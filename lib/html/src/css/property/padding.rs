use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Padding {
    pub dir: Direction,
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

impl From<Padding> for Property {
    fn from(padding: Padding) -> Property {
        Property::Padding(padding)
    }
}

#[cfg(test)]
mod padding_tests {

    use super::{Direction, Padding, Property, Render};

    #[test]
    fn render_padding_top() {
        let result = Padding {
            dir: Direction::Top,
        }
        .render();
        let expected = "padding-top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_padding_bot() {
        let result = Padding {
            dir: Direction::Bottom,
        }
        .render();
        let expected = "padding-bottom";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_padding_left() {
        let result = Padding {
            dir: Direction::Left,
        }
        .render();
        let expected = "padding-left";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_padding_right() {
        let result = Padding {
            dir: Direction::Right,
        }
        .render();
        let expected = "padding-right";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_padding_all() {
        let result = Padding {
            dir: Direction::All,
        }
        .render();
        let expected = "padding";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Padding as Into<Property>>::into(Padding {
            dir: Direction::All,
        })
        .render();
        let expected = Padding {
            dir: Direction::All,
        }
        .render();
        assert_eq!(result, expected)
    }
}
