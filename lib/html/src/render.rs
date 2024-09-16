pub trait Render {
    fn render(&self) -> String;
}

impl<T: Render> Render for Vec<T> {
    fn render(&self) -> String {
        let mut t_strs = vec![];
        for t in self.iter() {
            t_strs.push(t.render());
        }
        t_strs.join("\n")
    }
}
