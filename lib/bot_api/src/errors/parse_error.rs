use std::fmt;

#[derive(Clone, Debug)]
pub struct ParseError {
    ty: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        frmt.write_str(&format!("Could not parse {}", self.ty))
    }
}

impl From<url::ParseError> for ParseError {
    fn from(parse_err: url::ParseError) -> ParseError {
        ParseError {
            ty: parse_err.to_string(),
        }
    }
}
