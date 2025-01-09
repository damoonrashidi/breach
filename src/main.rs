use std::{cell::RefCell, error::Error, thread, time::Duration};

use breach::{
    entity::{projectiles::Bullet, Collidable},
    event::GameEvent,
    geometry::{Pos, Rect},
    state::State,
};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = crossterm::terminal::size()?;
    let mut state = State::new(Rect::new(&Pos(0.0, 0.0), width as f32, height as f32));
    let (tx, rx) = std::sync::mpsc::channel::<breach::event::Event>();

    thread::spawn(move || {
        let tx = tx.clone();
        while let Ok(event) = crossterm::event::read() {
            if let Some(action) = match event {
                Event::FocusLost => Some(breach::event::Event::Game(GameEvent::Pause)),
                Event::Resize(w, h) => Some(breach::event::Event::Game(GameEvent::Resize(w, h))),
                Event::Mouse(evt) => {
                    if let crossterm::event::MouseEvent {
                        kind: crossterm::event::MouseEventKind::Moved,
                        ..
                    } = evt
                    {
                        Some(breach::event::Event::Player(
                            breach::event::PlayerEvent::Aim(Pos(evt.column as f32, evt.row as f32)),
                        ))
                    } else {
                        None
                    }
                }
                Event::Key(key) => match key {
                    KeyEvent {
                        code: KeyCode::Char('q') | KeyCode::Esc,
                        ..
                    } => Some(breach::event::Event::Game(GameEvent::Quit)),
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        ..
                    } => Some(breach::event::Event::Player(
                        breach::event::PlayerEvent::Move(0.0, -1.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        ..
                    } => Some(breach::event::Event::Player(
                        breach::event::PlayerEvent::Move(-2.0, 0.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        ..
                    } => Some(breach::event::Event::Player(
                        breach::event::PlayerEvent::Move(0.0, 1.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        ..
                    } => Some(breach::event::Event::Player(
                        breach::event::PlayerEvent::Move(2.0, 0.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char(' '),
                        ..
                    } => Some(breach::event::Event::Player(
                        breach::event::PlayerEvent::Shoot,
                    )),
                    _ => None,
                },
                _ => None,
            } {
                tx.send(action).ok();
            }
        }
        drop(tx)
    });

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    loop {
        if let Ok(action) = rx.try_recv() {
            match action {
                breach::event::Event::Player(player_event) => {
                    match player_event {
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
                            state.player.borrow_mut().aim = player_pos.angle(&pos)
                        }
                        breach::event::PlayerEvent::Shoot => {
                            let player = state.player.borrow();
                            state.projectiles.borrow_mut().push(RefCell::new(Box::new(
                                Bullet::new(player.hitbox().center(), 1., player.aim),
                            )));
                        }
                    }
                }
                breach::event::Event::Game(game_event) => match game_event {
                    GameEvent::Pause => state.pause(),
                    GameEvent::Play => state.play(),
                    GameEvent::Resize(w, h) => {
                        state.canvas = Rect::new(&Pos(0., 0.), w as f32, h as f32)
                    }
                    GameEvent::Quit => break,
                },
            }
        }
        state.frame();
        state.render()?;
        thread::sleep(Duration::from_millis(8));
    }

    crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
