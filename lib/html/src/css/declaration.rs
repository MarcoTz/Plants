use super::{property::Property, value::Value};
use crate::render::Render;

#[derive(Clone, PartialEq, Debug)]
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

#[cfg(test)]
mod css_declaration_tests {
    use super::{Declaration, Render};
    use crate::css::{property::Property, value::Keyword};

    #[test]
    fn render_unimportant() {
        let result = Declaration {
            property: Property::TextAlign,
            value: Keyword::Center.into(),
            important: false,
        }
        .render();
        let expected = "text-align:center;";
        assert_eq!(result, expected);
    }

    #[test]
    fn render_important() {
        let result = Declaration {
            property: Property::Cursor,
            value: Keyword::Pointer.into(),
            important: true,
        }
        .render();
        let expected = "cursor:pointer!important;";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_from() {
        let decl: Declaration = (Property::Float, Keyword::Left.into()).into();
        let result = decl.render();
        let expected = "float:left;";
        assert_eq!(result, expected)
    }
}
