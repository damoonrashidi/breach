use crossterm::{cursor::MoveTo, style::Print};

use crate::render::Render;

#[derive(Debug)]
pub struct Map {
    level: [[char; 80]; 50],
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            level: [[' '; 80]; 50],
        }
    }
}

impl Render for Map {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for y in 0..self.level.len() {
            for x in 0..self.level[0].len() {
                crossterm::queue!(stdout, MoveTo(x as u16, y as u16), Print(self.level[y][x]))?;
            }
        }

        Ok(())
    }
}
