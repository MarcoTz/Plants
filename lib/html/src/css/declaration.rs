use super::{property::Property, value::Value};
use crate::render::Render;

#[derive(Clone)]
pub struct Declaration {
    pub property: Property,
    pub value: Value,
    pub important: bool,
}

impl Render for Declaration {
    fn render(&self) -> String {
        let property_str = self.property.render();
        let value_str = self.value.render();
        let imp_str = if self.important {
            "!important".to_owned()
        } else {
            "".to_owned()
        };
        format!("{property_str}:{value_str}{imp_str};")
    }
}

impl From<(Property, Value)> for Declaration {
    fn from((prop, val): (Property, Value)) -> Declaration {
        Declaration {
            property: prop,
            value: val,
            important: false,
        }
    }
}
