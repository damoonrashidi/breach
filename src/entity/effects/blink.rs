use crate::render::Render;
use crate::{
    entity::{effect::Effect, Entity},
    geometry::Pos,
};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;

#[derive(Debug)]
pub struct BlinkEffect {
    frames: &'static str,
    from: Pos,
    to: Pos,
    frame: u8,
}

impl BlinkEffect {
    pub fn new(from: Pos, to: Pos) -> Self {
        Self {
            from,
            to,
            frames: "OOOOOOOOOOOOOOOOOOOoooooooooooooooooooooooooo.............",
            frame: 0,
        }
    }
}

impl Effect for BlinkEffect {
    fn is_done(&self) -> bool {
        self.frame >= self.frames.len() as u8
    }
}

impl Entity for BlinkEffect {
    fn id(&self) -> String {
        "effect:blink".to_string()
    }

    fn update(&mut self, _: &crate::state::State) {
        self.frame += 1;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Render for BlinkEffect {
    fn render(
        &self,
        stdout: &mut std::io::Stdout,
    ) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        let alpha = self.from.angle(&self.to);

        for i in 0..3 {
            let x = self.from.0 + alpha.cos() * (i as f32 * 2.0);
            let y = self.from.1 + alpha.sin() * i as f32;
            let frame = self.frame.saturating_sub(i * 5) as usize;
            let chr = self.frames.get(frame..frame + 1).unwrap_or("+");

            crossterm::queue!(stdout, MoveTo(x as u16, y as u16), Print(chr))?;
        }

        Ok(())
    }
}
