/// Server events
pub enum Event {
    Info(String),
    Start(bool),
    Stop(bool),
    Crash(),
    Player(String, String),
}