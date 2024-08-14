use crate::render::Render;

#[derive(Clone)]
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
