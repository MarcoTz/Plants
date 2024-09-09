use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub content: Rc<HtmlElement>,
}

#[derive(Debug, PartialEq)]
pub struct Select {
    pub attributes: Vec<Attribute>,
    pub options: Vec<SelectOption>,
}

impl Render for Select {
    fn render(&self) -> String {
        let attr_str = self.attributes.render().replace('\n', " ");
        let option_strs: Vec<String> = self.options.iter().map(|opt| opt.render()).collect();
        format!(
            "<select {attr_str}>\n\t{}\n</select>",
            option_strs.join("\n\t")
        )
    }
}
impl Render for SelectOption {
    fn render(&self) -> String {
        let content_str = self.content.render();
        let value_str = self.value.clone();
        format!("<option value=\"{value_str}\">{content_str}</option>")
    }
}

impl From<Select> for HtmlElement {
    fn from(select: Select) -> HtmlElement {
        HtmlElement::Select(select)
    }
}

#[cfg(test)]
mod select_tests {

    use super::{Attribute, HtmlElement, Render, Select, SelectOption};
    use std::rc::Rc;

    fn example_option() -> SelectOption {
        SelectOption {
            value: "option1".to_owned(),
            content: Rc::new("option one".to_owned().into()),
        }
    }

    fn example_select() -> Select {
        Select {
            attributes: vec![Attribute::Id("selector".to_owned())],
            options: vec![
                example_option(),
                SelectOption {
                    value: "option2".to_owned(),
                    content: Rc::new("option two".to_owned().into()),
                },
            ],
        }
    }

    #[test]
    fn render_select() {
        let result = example_select().render();
        let expected = "<select id=\"selector\">\n\t<option value=\"option1\">option one</option>\n\t<option value=\"option2\">option two</option>\n</select>";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_into() {
        let result = <Select as Into<HtmlElement>>::into(example_select()).render();
        let expected = example_select().render();
        assert_eq!(result, expected)
    }

    #[test]
    fn render_option() {
        let result = example_option().render();
        let expected = "<option value=\"option1\">option one</option>";
        assert_eq!(result, expected)
    }
}
