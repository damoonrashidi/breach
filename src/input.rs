use std::sync::mpsc::Sender;

use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{event::GameEvent, geometry::Pos};

pub fn handle_input(tx: Sender<crate::event::Event>) {
    std::thread::spawn(move || {
        while let Ok(event) = crossterm::event::read() {
            if let Some(action) = match event {
                Event::FocusLost => Some(crate::event::Event::Game(GameEvent::Pause)),
                Event::Resize(w, h) => Some(crate::event::Event::Game(GameEvent::Resize(w, h))),
                Event::Mouse(evt) => match evt {
                    crossterm::event::MouseEvent {
                        kind:
                            crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Shoot,
                    )),
                    crossterm::event::MouseEvent {
                        kind:
                            crossterm::event::MouseEventKind::Drag(crossterm::event::MouseButton::Left)
                            | crossterm::event::MouseEventKind::Moved,
                        ..
                    } => Some(crate::event::Event::Player(crate::event::PlayerEvent::Aim(
                        Pos(evt.column.into(), evt.row.into()),
                    ))),
                    _ => None,
                },
                Event::Key(key) => match key {
                    KeyEvent {
                        code: KeyCode::Char('q') | KeyCode::Esc,
                        ..
                    } => Some(crate::event::Event::Game(GameEvent::Quit)),
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Move(0.0, -1.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Move(-2.0, 0.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Move(0.0, 1.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Move(2.0, 0.0),
                    )),
                    KeyEvent {
                        code: KeyCode::Char('p'),
                        ..
                    } => Some(crate::event::Event::Game(crate::event::GameEvent::Pause)),
                    KeyEvent {
                        code: KeyCode::Char('e'),
                        ..
                    } => Some(crate::event::Event::Player(
                        crate::event::PlayerEvent::Ability(crate::entity::player::Ability::Blink),
                    )),
                    _ => None,
                },
                _ => None,
            } {
                tx.send(action).ok();
            }
        }
        drop(tx);
    });
}
