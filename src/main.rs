use std::{error::Error, thread, time::Duration};

use breach::{
    entity::{effects::blink::BlinkEffect, projectiles::Bullet, Collidable},
    event::GameEvent,
    geometry::{Pos, Rect},
    state::State,
};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = crossterm::terminal::size()?;
    let mut state = State::new(Rect::new(
        &Pos(0.0, 0.0),
        f32::from(width),
        f32::from(height),
    ));
    let (tx, rx) = std::sync::mpsc::channel::<breach::event::Event>();

    breach::input::handle_input(tx.clone());

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    loop {
        if let Ok(action) = rx.try_recv() {
            match action {
                breach::event::Event::Player(player_event) => match player_event {
                    breach::event::PlayerEvent::Move(x, y) => {
                        let hbox = state.player.borrow().hitbox();
                        let old = state.player.borrow().pos;
                        let new = Pos(
                            (old.0 + x).clamp(0.0, state.canvas.w - hbox.w),
                            (old.1 + y).clamp(0.0, state.canvas.h - hbox.h),
                        );
                        state.player.borrow_mut().pos = new;
                    }
                    breach::event::PlayerEvent::Aim(pos) => {
                        let player_pos = state.player.borrow().pos;
                        state.player.borrow_mut().aim = player_pos.angle(&pos);
                    }
                    breach::event::PlayerEvent::Shoot => {
                        let player = state.player.borrow();
                        let bullet = Bullet::new(player.hitbox().center(), 1., player.aim);
                        state.spawn_projectile(bullet);
                    }
                    breach::event::PlayerEvent::Ability(_) => {
                        let aim = state.player.borrow().aim;
                        let c = state.player.borrow().hitbox().center();
                        let x = c.0 + aim.cos() * 10.0;
                        let y = c.1 + aim.sin() * 5.0;
                        state.spawn_effect(BlinkEffect::new(c, Pos(x, y)));
                        state.player.borrow_mut().pos =
                            Pos(x.clamp(0., state.canvas.w), y.clamp(0., state.canvas.h));
                    }
                },
                breach::event::Event::Game(game_event) => match game_event {
                    GameEvent::Pause => state.pause(),
                    GameEvent::Play => state.play(),
                    GameEvent::Resize(w, h) => {
                        state.canvas = Rect::new(&Pos(0., 0.), f32::from(w), f32::from(h));
                    }
                    GameEvent::Quit => break,
                },
            }
        }
        state.frame();
        state.render()?;
        thread::sleep(Duration::from_millis(16));
    }

    crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
