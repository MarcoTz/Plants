pub trait Command: Sized {
    type Error: std::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Error>;
}
