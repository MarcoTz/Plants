use super::errors::Error;
use std::fs::File;
use std::io::prelude::Write;

pub fn write_html(html_content: String, file_path: &str) -> Result<(), Error> {
    let mut out_file = File::create(file_path)?;
    out_file.write_all(html_content.as_bytes())?;
    Ok(())
}
