use crossterm::{cursor::MoveTo, style::Print};

use crate::{geometry::Pos, render::Render};
use std::fmt::Display;

#[derive(Debug)]
pub struct Map {
    pub start_pos: Pos,
    pub level: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => '#',
            Tile::Floor => '-',
        };

        write!(f, "{c}")
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            ' ' => Ok(Tile::Floor),
            _ => Err("Invalid tile character"),
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let tile_chars: Vec<Vec<_>> = value.lines().map(|line| line.chars().collect()).collect();
        let w = tile_chars[0].len();
        let h = tile_chars.len();
        let mut start_pos = Pos(0., 0.);

        let mut level = vec![vec![Tile::Floor; w]; h];

        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                level[y][x] = tile_chars[y][x].try_into().unwrap_or(Tile::Floor);
                if tile_chars[y][x] == '@' {
                    start_pos = Pos(x as f32, y as f32);
                }
            }
        }

        Self { start_pos, level }
    }
}

impl Render for Map {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for y in 0..self.level.len() {
            for x in 0..self.level[0].len() {
                if self.level[y][x] == Tile::Wall {
                    crossterm::queue!(stdout, MoveTo(x as u16, y as u16), Print(self.level[y][x]))?;
                }
            }
        }

        Ok(())
    }
}
