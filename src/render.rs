use std::{error::Error, io::Stdout};

pub trait Render {
    /**
    Attempts to render an Entity
    # Errors
    if the entity cannot be rendered
    */
    fn render(&self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>>;
}
