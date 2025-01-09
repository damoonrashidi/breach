#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Player(PlayerEvent),
    Game(GameEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    Pause,
    Play,
    Resize(u16, u16),
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerEvent {
    Move(f32, f32),
}
