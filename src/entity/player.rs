use super::{enemy::Enemy, projectile::Projectile, Collidable, Entity};
use crate::{
    geometry::{Pos, Rect},
    render::Render,
};
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::error::Error;

#[derive(Debug)]
pub struct Player {
    hp: u32,
    pub pos: Pos,
    pub aim: f32,
}

impl Player {
    pub fn new(pos: Pos) -> Self {
        Self {
            hp: 100,
            pos,
            aim: 0.,
        }
    }
}

impl Entity for Player {
    fn id(&self) -> String {
        "player".to_string()
    }

    fn update(&mut self, _: &crate::state::State) {}
}

impl Render for Player {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn Error>> {
        let (x, y) = self.hitbox().center().into();
        crossterm::queue!(
            stdout,
            SetForegroundColor(Color::Red),
            MoveTo(x - 1, y - 1),
            Print("╭━╮"),
            MoveTo(x - 1, y),
            Print("╰━╯"),
            ResetColor,
        )?;
        Ok(())
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Rect {
        Rect::new(&self.pos, 3., 3.)
    }

    fn on_hit(&mut self, other: &dyn Collidable, _: &mut crate::state::State) {
        if let Some(projectile) = other.as_any().downcast_ref::<Box<dyn Projectile>>() {
            self.hp = self.hp.saturating_sub(projectile.dmg());
        }
        if let Some(enemy) = other.as_any().downcast_ref::<Box<dyn Enemy>>() {
            self.hp = self.hp.saturating_sub(enemy.dmg());
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
