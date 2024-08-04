use super::errors::Error;
use super::pages::Page;
use minijinja::Environment;
use std::fs;

pub struct Renderer<'a> {
    env: Environment<'a>,
    template_dir: String,
}

impl<'a> Renderer<'a> {
    fn load_template(&mut self, page: &Page) -> Result<(), Error> {
        let page_name = page.get_page_name();
        let page_contents = fs::read_to_string(page.get_page_path(&self.template_dir))?;
        self.env.add_template_owned(page_name, page_contents)?;
        Ok(())
    }

    fn load_templates(&mut self) -> Result<(), Error> {
        for page in Page::iterator() {
            self.load_template(page)?;
        }
        Ok(())
    }
}
