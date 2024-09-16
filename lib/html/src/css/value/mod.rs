mod color;
mod keyword;
mod unit;

use crate::render::Render;
pub use color::Color;
pub use keyword::Keyword;
pub use unit::Unit;

#[derive(Clone, PartialEq, Debug)]
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

#[cfg(test)]
mod value_tests {
    use super::{Color, Keyword, Render, Unit, Value};

    #[test]
    fn render_color() {
        let result = Value::Color(Color::Rgb(1, 2, 3)).render();
        let expected = "#010203";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_str() {
        let result = Value::Str("value".to_owned()).render();
        let expected = "\"value\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_var() {
        let result = Value::Var("bg-color".to_owned()).render();
        let expected = "var(--bg-color)";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_measurement() {
        let result = Value::Measurement(1.0, Unit::Em).render();
        let expected = "1em";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_keyword() {
        let result = Value::Keyword(Keyword::Center).render();
        let expected = "center";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <(f32, Unit) as Into<Value>>::into((1.0, Unit::Em)).render();
        let expected = Value::Measurement(1.0, Unit::Em).render();
        assert_eq!(result, expected)
    }
}
