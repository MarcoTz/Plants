use crate::render::Render;
pub enum Unit {
    Pt,
    Percent,
    Em,
    Vh,
}

impl Render for Unit {
    fn render(&self) -> String {
        match self {
            Unit::Pt => "pt".to_owned(),
            Unit::Percent => "%".to_owned(),
            Unit::Em => "em".to_owned(),
            Unit::Vh => "vh".to_owned(),
        }
    }
}
