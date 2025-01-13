use std::{cell::RefCell, error::Error, fmt::Display, io::Write};

use crossterm::{
    cursor::MoveTo,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::Clear,
};

use crate::{
    entity::{
        effect::Effect, enemies::Goblo, enemy::Enemy, player::Player, projectile::Projectile,
        Entity,
    },
    geometry::Pos,
    map::Map,
    render::Render,
};

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Play,
    Pause,
}
#[derive(Debug)]
pub struct State {
    pub mode: RefCell<GameMode>,
    pub map: RefCell<Map>,
    pub canvas: crate::geometry::Rect,
    pub player: RefCell<crate::entity::player::Player>,
    pub enemies: RefCell<Vec<RefCell<Box<dyn crate::entity::enemy::Enemy>>>>,
    pub projectiles: RefCell<Vec<RefCell<Box<dyn crate::entity::projectile::Projectile>>>>,
    pub effects: RefCell<Vec<RefCell<Box<dyn crate::entity::effect::Effect>>>>,
    pub log: RefCell<Option<String>>,
}

impl State {
    #[must_use]
    pub fn new(canvas: crate::geometry::Rect) -> Self {
        let map = Map::from(include_str!("map/levels/level_1.txt"));
        let player_pos = map.start_pos;

        Self {
            mode: RefCell::new(GameMode::Play),
            map: RefCell::new(map),
            canvas,
            player: RefCell::new(Player::new(player_pos)),
            enemies: RefCell::new(vec![RefCell::new(Box::new(Goblo::new(Pos(10., 10.))))]),
            projectiles: RefCell::new(vec![]),
            effects: RefCell::new(vec![]),
            log: RefCell::new(None),
        }
    }

    pub fn play(&mut self) {
        self.mode = RefCell::new(GameMode::Play);
    }

    pub fn pause(&mut self) {
        self.mode = RefCell::new(GameMode::Pause);
    }

    pub fn log(&self, msg: impl Display) {
        *self.log.borrow_mut() = Some(msg.to_string());
    }

    pub fn spawn_enemy(&self, enemy: impl Enemy + 'static) {
        self.enemies
            .borrow_mut()
            .push(RefCell::new(Box::new(enemy)));
    }

    pub fn spawn_projectile(&self, projectile: impl Projectile + 'static) {
        self.projectiles
            .borrow_mut()
            .push(RefCell::new(Box::new(projectile)));
    }

    pub fn spawn_effect(&self, effect: impl Effect + 'static) {
        self.effects
            .borrow_mut()
            .push(RefCell::new(Box::new(effect)));
    }

    pub fn frame(&self) {
        self.player.borrow_mut().update(self);

        for enemy in self.enemies.borrow_mut().iter() {
            enemy.borrow_mut().update(self);
        }

        for projectile in self.projectiles.borrow_mut().iter() {
            projectile.borrow_mut().update(self);
        }

        for enemy in self.enemies.borrow_mut().iter() {
            let mut enemy = enemy.borrow_mut();
            let enemy_hitbox = enemy.hitbox();

            for projectile in self.projectiles.borrow_mut().iter() {
                let mut projectile = projectile.borrow_mut();
                if enemy_hitbox.intersects(&projectile.hitbox()) {
                    enemy.on_hit(Box::new(projectile.as_ref()), self);
                    projectile.on_hit(Box::new(enemy.as_ref()), self);
                }
            }
        }

        for effect in self.effects.borrow_mut().iter() {
            effect.borrow_mut().update(self);
        }

        self.enemies
            .borrow_mut()
            .retain_mut(|enemy| enemy.borrow().is_alive());

        self.effects
            .borrow_mut()
            .retain_mut(|effect| !effect.borrow().is_done());

        self.projectiles
            .borrow_mut()
            .retain_mut(|projectile| self.canvas.contains(projectile.borrow().pos()));
    }

    /**
    Renders the entire game state for the current frame

    # Errors
    Will error out if any of the objects cannot be rendered
    */
    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = std::io::stdout();

        crossterm::queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;

        self.map.borrow().render(&mut stdout)?;

        for enemy in self.enemies.borrow().iter() {
            enemy.borrow().render(&mut stdout)?;
        }

        for projectile in self.projectiles.borrow().iter() {
            projectile.borrow().render(&mut stdout)?;
        }

        for effect in self.effects.borrow().iter() {
            effect.borrow().render(&mut stdout)?;
        }

        self.player.borrow().render(&mut stdout)?;

        let player = self.player.borrow();

        let c_x = player.pos.0 + player.aim.cos() * 10.;
        let c_y = player.pos.1 + player.aim.sin() * 5.;

        let (c_x, c_y) = Pos(c_x, c_y).into();

        crossterm::queue!(
            stdout,
            MoveTo(c_x, c_y),
            SetForegroundColor(crossterm::style::Color::Magenta),
            Print('⌖'),
            ResetColor
        )?;

        let msg = self.log.borrow().clone().unwrap_or_default();
        crossterm::queue!(stdout, MoveTo(0, (self.canvas.h - 1.0) as u16), Print(msg))?;

        stdout.flush()?;

        Ok(())
    }
}
