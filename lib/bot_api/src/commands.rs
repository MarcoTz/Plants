pub trait Command: Sized {
    type Error: std::error::Error;
    fn parse(s: &str) -> Result<Self, Self::Error>;
    fn get_description(&self) -> String;
}
