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
    #[must_use]
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
            let x: f32 = self.from.0 + alpha.cos() * f32::from(i) * 2.0;
            let y: f32 = self.from.1 + alpha.sin() * f32::from(i);
            let frame = self.frame.saturating_sub(i * 5) as usize;
            let chr = self.frames.get(frame..=frame).unwrap_or("+");

            crossterm::queue!(stdout, MoveTo(x as u16, y as u16), Print(chr))?;
        }

        Ok(())
    }
}
