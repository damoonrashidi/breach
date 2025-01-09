use crossterm::{cursor::MoveTo, style::Print};

use crate::{geometry::Pos, render::Render};

use super::{projectile::Projectile, Collidable, Entity};

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pos: Pos,
    angle: f32,
    vel: f32,
}

impl Bullet {
    pub fn new(pos: Pos, vel: f32, angle: f32) -> Self {
        Self { pos, vel, angle }
    }
}

impl Render for Bullet {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let (x, y) = self.pos.into();
        crossterm::queue!(stdout, MoveTo(x, y), Print('.'))?;

        Ok(())
    }
}

impl Projectile for Bullet {
    fn dmg(&self) -> u32 {
        1
    }

    fn pos(&self) -> &Pos {
        &self.pos
    }
}

impl Entity for Bullet {
    fn id(&self) -> String {
        "Bullet".to_string()
    }

    fn update(&mut self, _: &crate::state::State) {
        let x = self.pos.0 + self.angle.cos() * self.vel;
        let y = self.pos.1 + self.angle.sin() * self.vel;

        self.pos = Pos(x, y);
    }
}

impl Collidable for Bullet {
    fn hitbox(&self) -> crate::geometry::Rect {
        crate::geometry::Rect {
            pos: self.pos,
            w: 1.,
            h: 1.,
        }
    }

    fn on_hit(&mut self, _other: &dyn Collidable, _state: &mut crate::state::State) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
