use crate::render::Render;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Unit {
    Pt,
    Percent,
    Em,
    Vh,
    Px,
}

impl Render for Unit {
    fn render(&self) -> String {
        match self {
            Unit::Pt => "pt".to_owned(),
            Unit::Percent => "%".to_owned(),
            Unit::Em => "em".to_owned(),
            Unit::Vh => "vh".to_owned(),
            Unit::Px => "px".to_owned(),
        }
    }
}

#[cfg(test)]
mod unit_tests {

    use super::{Render, Unit};

    #[test]
    fn render_pt() {
        let result = Unit::Pt.render();
        let expected = "pt";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_percent() {
        let result = Unit::Percent.render();
        let expected = "%";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_em() {
        let result = Unit::Em.render();
        let expected = "em";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_vh() {
        let result = Unit::Vh.render();
        let expected = "vh";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_px() {
        let result = Unit::Px.render();
        let expected = "px";
        assert_eq!(result, expected)
    }
}
