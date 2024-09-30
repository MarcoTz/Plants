use super::render::Render;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribute {
    Id(String),
    Class(Vec<String>),
    Src(String),
    Style(String),
    Href(String),
    Rel(String),
    Type(String),
    OnChange(String),
    OnLoad(String),
    OnKeyUp(String),
    OnClick(String),
}

impl Render for Attribute {
    fn render(&self) -> String {
        match self {
            Attribute::Id(id) => format!("id=\"{id}\""),
            Attribute::Class(classes) => {
                let class_str = classes.join(" ");
                format!("class=\"{class_str}\"")
            }
            Attribute::Src(src) => format!("src=\"{src}\""),
            Attribute::OnKeyUp(keyup) => format!("onKeyUp=\"{keyup}\""),
            Attribute::Style(style) => format!("style=\"{style}\""),
            Attribute::Href(href) => format!("href=\"{href}\""),
            Attribute::Type(ty) => format!("type=\"{ty}\""),
            Attribute::OnChange(onchange) => format!("onChange=\"{onchange}\""),
            Attribute::OnLoad(onload) => format!("onLoad=\"{onload}\""),
            Attribute::OnClick(onclick) => format!("onClick=\"{onclick}\""),
            Attribute::Rel(rel) => format!("rel=\"{rel}\""),
        }
    }
}

#[cfg(test)]
mod attribute_tests {
    use super::{Attribute, Render};

    #[test]
    fn render_id() {
        let result = Attribute::Id("test_id".to_owned()).render();
        let expected = "id=\"test_id\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_class() {
        let result = Attribute::Class(vec!["class1".to_owned(), "class2".to_owned()]).render();
        let expected = "class=\"class1 class2\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_src() {
        let result = Attribute::Src("file.jpg".to_owned()).render();
        let expected = "src=\"file.jpg\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_style() {
        let result = Attribute::Style("height:100%;".to_owned()).render();
        let expected = "style=\"height:100%;\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_href() {
        let result = Attribute::Href("index.html".to_owned()).render();
        let expected = "href=\"index.html\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_rel() {
        let result = Attribute::Rel("stylesheet".to_owned()).render();
        let expected = "rel=\"stylesheet\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_type() {
        let result = Attribute::Type("javascript".to_owned()).render();
        let expected = "type=\"javascript\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_onchange() {
        let result = Attribute::OnChange("alert()".to_owned()).render();
        let expected = "onChange=\"alert()\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_onload() {
        let result = Attribute::OnLoad("alert()".to_owned()).render();
        let expected = "onLoad=\"alert()\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_onkeyup() {
        let result = Attribute::OnKeyUp("alert()".to_owned()).render();
        let expected = "onKeyUp=\"alert()\"";
        assert_eq!(result, expected)
    }

    #[test]
    fn render_onclick() {
        let result = Attribute::OnClick("alert()".to_owned()).render();
        let expected = "onClick=\"alert()\"";
        assert_eq!(result, expected)
    }
}
