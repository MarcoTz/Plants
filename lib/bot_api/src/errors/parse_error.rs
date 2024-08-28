use std::fmt;

#[derive(Clone)]
pub struct ParseError {
    ty: String,
}

impl fmt::Debug for ParseError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str("")
    }
}

impl From<url::ParseError> for ParseError {
    fn from(parse_err: url::ParseError) -> ParseError {
        ParseError {
            ty: parse_err.to_string(),
        }
    }
}
