use super::render::Render;

pub enum Attribute {
    Id(String),
    Class(Vec<String>),
    Src(String),
    OnKeyUp(String),
    Style(String),
    Href(String),
    Type(String),
    OnChange(String),
    Rel(String),
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
            Attribute::Rel(rel) => format!("rel=\"{rel}\""),
        }
    }
}
