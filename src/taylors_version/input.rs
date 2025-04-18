use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};

use super::level::position::Direction;

pub enum GameInput {
    Direction(Direction),
    Break,
    Quit,
}

impl TryFrom<KeyEvent> for GameInput {
    type Error = KeyEvent;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match value.code {
            KeyCode::Up => Ok(Self::Direction(Direction::Up)),
            KeyCode::Down => Ok(Self::Direction(Direction::Down)),
            KeyCode::Left => Ok(Self::Direction(Direction::Left)),
            KeyCode::Right => Ok(Self::Direction(Direction::Right)),
            KeyCode::Char('q') => Ok(Self::Quit),
            KeyCode::Char('c') if value.modifiers.contains(KeyModifiers::CONTROL) => Ok(Self::Break),
            _ => Err(value)
        }
    }
}

pub enum MenuInput {
    NewGame,
    Quit
}

impl TryFrom<KeyEvent> for MenuInput {
    type Error = KeyEvent;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match value.code {
            KeyCode::Enter => Ok(Self::NewGame),
            KeyCode::Char('q') => Ok(Self::Quit),
            KeyCode::Char('c') if value.modifiers.contains(KeyModifiers::CONTROL) => Ok(Self::Quit),
            _ => Err(value)
        }
    }
}

fn in_raw_mode<T>(fun: impl Fn() -> T) -> T {
    enable_raw_mode().expect("Failed to enable raw mode");
    let out = fun();
    disable_raw_mode().expect("Failed to disable raw mode");
    out
}

pub fn wait_for_input<I: TryFrom<KeyEvent>>() -> Option<I> {
    in_raw_mode(|| {
        match event::read() {
            Ok(Event::Key(key_event)) => I::try_from(key_event).ok(),
            Ok(_) | Err(_) => None
        }
    })
}