use std::{cell::RefCell, error::Error, io::Write};

use crossterm::{cursor::Hide, terminal::Clear};

use crate::{
    entity::{enemies::Goblo, player::Player, Entity},
    geometry::Pos,
    render::Render,
};

#[derive(Debug)]
pub enum GameMode {
    Play,
    Pause,
}

#[derive(Debug)]
pub struct State {
    pub mode: RefCell<GameMode>,
    pub canvas: crate::geometry::Rect, // Immutable, no RefCell needed
    pub player: RefCell<crate::entity::player::Player>,
    pub enemies: RefCell<Vec<RefCell<Box<dyn crate::entity::enemy::Enemy>>>>,
    pub projectiles: RefCell<Vec<RefCell<Box<dyn crate::entity::projectile::Projectile>>>>,
    // pub projectiles: Vec<Box<dyn Projectile>>,
    // pub effects: Vec<Box<dyn Effect>>,
    pub log: RefCell<Option<String>>,
}

impl State {
    pub fn new(canvas: crate::geometry::Rect) -> Self {
        let player_pos = canvas.center();
        Self {
            mode: RefCell::new(GameMode::Play),
            canvas,
            player: RefCell::new(Player::new(player_pos)),
            enemies: RefCell::new(vec![RefCell::new(Box::new(Goblo::new(Pos(10., 10.))))]),
            projectiles: RefCell::new(vec![]),
            log: RefCell::new(None),
        }
    }

    pub fn play(&mut self) {
        self.mode = RefCell::new(GameMode::Play);
    }

    pub fn pause(&mut self) {
        self.mode = RefCell::new(GameMode::Pause);
    }

    pub fn frame(&self) {
        self.player.borrow_mut().update(self);

        for enemy in self.enemies.borrow_mut().iter() {
            enemy.borrow_mut().update(self);
        }

        for projectile in self.projectiles.borrow_mut().iter() {
            projectile.borrow_mut().update(self);
        }

        self.enemies
            .borrow_mut()
            .retain_mut(|enemy| enemy.borrow_mut().is_alive());
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = std::io::stdout();

        crossterm::execute!(stdout, Hide, Clear(crossterm::terminal::ClearType::All))?;

        for enemy in self.enemies.borrow().iter() {
            enemy.borrow().render(&mut stdout)?;
        }

        self.player.borrow().render(&mut stdout)?;

        stdout.flush()?;

        Ok(())
    }
}
