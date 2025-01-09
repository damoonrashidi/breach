use crate::geometry::Pos;

#[derive(Debug, Clone)]
pub enum Event {
    Player(PlayerEvent),
    Game(GameEvent),
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    Pause,
    Play,
    Resize(u16, u16),
    Quit,
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Move(f32, f32),
    Aim(Pos),
    Shoot,
}
