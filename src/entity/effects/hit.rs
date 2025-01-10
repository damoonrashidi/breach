use crossterm::{cursor::MoveTo, style::Print};

use crate::{
    entity::{effect::Effect, Entity},
    geometry::Pos,
    render::Render,
};

#[derive(Debug, Clone)]
pub struct HitEffect {
    pos: Pos,
    dmg: u16,
    frame: u8,
}

impl Effect for HitEffect {
    fn is_done(&self) -> bool {
        self.frame < 5
    }
}

impl Entity for HitEffect {
    fn id(&self) -> String {
        "effect:hit".to_string()
    }

    fn update(&mut self, _: &crate::state::State) {
        self.pos.1 -= 1.0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Render for HitEffect {
    fn render(
        &self,
        stdout: &mut std::io::Stdout,
    ) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        let (x, y) = self.pos.into();
        crossterm::queue!(stdout, MoveTo(x, y), Print(self.dmg))?;

        Ok(())
    }
}
