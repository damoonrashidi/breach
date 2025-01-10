use super::{
    effects::hit::HitEffect, enemy::Enemy, projectile::Projectile, projectiles::Bullet, Collidable,
    Entity,
};
use crate::{geometry::Pos, render::Render};
use crossterm::{cursor::MoveTo, style::Print};

#[derive(Debug, Clone, Copy)]
pub struct Goblo {
    hp: u32,
    pos: Pos,
}

impl Goblo {
    pub fn new(pos: Pos) -> Self {
        Self { hp: 10, pos }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Render for Goblo {
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let (x, y) = self.pos.into();
        crossterm::queue!(stdout, MoveTo(x, y), Print("G"))?;

        Ok(())
    }
}

impl Collidable for Goblo {
    fn hitbox(&self) -> crate::geometry::Rect {
        crate::geometry::Rect {
            pos: self.pos,
            w: 2.,
            h: 2.,
        }
    }

    fn on_hit(&mut self, other: Box<&dyn Collidable>, state: &crate::state::State) {
        if let Some(projectile) = (*other).as_any().downcast_ref::<Bullet>() {
            self.hp = self.hp.saturating_sub(projectile.dmg());
            let effect = HitEffect::new(&self.pos, projectile.dmg());
            state.spawn_effect(effect);
        }
    }
}
