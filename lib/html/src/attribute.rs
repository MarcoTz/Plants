use super::render::Render;

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
