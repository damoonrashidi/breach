use std::{error::Error, io::Stdout};

pub trait Render {
    fn render(&self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>>;
}
