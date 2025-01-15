use crossterm::{cursor::MoveTo, style::Print};

use crate::{
    geometry::{Pos, Rect},
    render::Render,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct Map {
    pub start_pos: Pos,
    pub level: Vec<Tile>,
    pub seen: Vec<Tile>,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub rect: Rect,
    kind: TileType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.kind {
            TileType::Wall => '#',
        };

        write!(f, "{c}")
    }
}

impl TryFrom<char> for TileType {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(TileType::Wall),
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

        let mut level = vec![];

        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                if let Ok(kind) = tile_chars[y][x].try_into() {
                    let rect = Rect::new(&Pos(x as f32, y as f32), 1., 1.);
                    level.push(Tile { rect, kind });
                    if tile_chars[y][x] == '@' {
                        start_pos = Pos(x as f32, y as f32);
                    }
                }
            }
        }

        Self {
            start_pos,
            level,
            seen: vec![],
        }
    }
}

impl Render for Map {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for tile in &self.level {
            if tile.kind == TileType::Wall {
                crossterm::queue!(
                    stdout,
                    MoveTo(tile.rect.pos.0 as u16, tile.rect.pos.1 as u16),
                    Print(tile)
                )?;
            }
        }

        Ok(())
    }
}
