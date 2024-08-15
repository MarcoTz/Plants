pub mod color;
pub mod keyword;
pub mod unit;

use crate::render::Render;
use color::Color;
use keyword::Keyword;
use unit::Unit;

#[derive(Clone)]
pub enum Value {
    Color(Color),
    Str(String),
    Var(String),
    Measurement(f32, Unit),
    Keyword(Keyword),
}

impl Render for Value {
    fn render(&self) -> String {
        match self {
            Value::Color(color) => color.render(),
            Value::Str(st) => format!("\"{st}\""),
            Value::Var(v) => format!("var(--{v})"),
            Value::Measurement(num, unit) => {
                let unit_str = unit.render();
                let num_str = format!("{num:.2}").replace(".00", "");
                format!("{num_str}{unit_str}")
            }
            Value::Keyword(kw) => kw.render(),
        }
    }
}

impl From<(f32, Unit)> for Value {
    fn from((val, unit): (f32, Unit)) -> Value {
        Value::Measurement(val, unit)
    }
}
