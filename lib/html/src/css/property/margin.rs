use super::{Direction, Property};
use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Margin {
    pub dir: Direction,
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

impl From<Margin> for Property {
    fn from(margin: Margin) -> Property {
        Property::Margin(margin)
    }
}

#[cfg(test)]
mod margin_tests {
    use super::{Direction, Margin, Property, Render};

    #[test]
    fn render_margin_top() {
        let result = Margin {
            dir: Direction::Top,
        }
        .render();
        let expected = "margin-top";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_margin_bot() {
        let result = Margin {
            dir: Direction::Bottom,
        }
        .render();
        let expected = "margin-bottom";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_margin_left() {
        let result = Margin {
            dir: Direction::Left,
        }
        .render();
        let expected = "margin-left";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_margin_right() {
        let result = Margin {
            dir: Direction::Right,
        }
        .render();
        let expected = "margin-right";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_margin_all() {
        let result = Margin {
            dir: Direction::All,
        }
        .render();
        let expected = "margin";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let margin = Margin {
            dir: Direction::Top,
        };
        let result = <Margin as Into<Property>>::into(margin.clone()).render();
        let expected = margin.render();
        assert_eq!(result, expected)
    }
}
