use super::{enemy::Enemy, Collidable, Entity};
use crate::{geometry::Pos, render::Render};
use crossterm::{cursor::MoveTo, style::Print};

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Goblo {
    hp: u16,
    pos: Pos,
}

impl Goblo {
    pub fn new(pos: Pos) -> Self {
        Self { hp: 254, pos }
    }
}

impl Enemy for Goblo {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn dmg(&self) -> u32 {
        todo!()
    }
}

impl Entity for Goblo {
    fn id(&self) -> String {
        "goblo".to_string()
    }

    fn update(&mut self, state: &crate::state::State) {
        let alpha = self.pos.angle(&state.player.borrow().hitbox().center());
        self.pos = Pos(
            self.pos.0 + alpha.cos() * 0.2,
            self.pos.1 + alpha.sin() * 0.1,
        );
    }
}

impl Render for Goblo {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let (x, y) = self.pos.into();
        crossterm::queue!(stdout, MoveTo(x, y), Print("G"))?;

        Ok(())
    }
}
