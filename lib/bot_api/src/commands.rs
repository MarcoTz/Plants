use std::error::Error;

pub trait Command: Sized {
    fn parse(s: &str) -> Result<Self, Box<dyn Error>>;
    fn get_description(&self) -> String;
}
