use crate::core::Direction;
use crossterm::event::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameAction {
    Move(Direction),
    Fire,
    Pause,
    Restart,
    Quit,
}

pub trait InputMapper {
    fn map(&self, code: KeyCode) -> Option<GameAction>;
}

pub struct DefaultInputMapper;

impl InputMapper for DefaultInputMapper {
    fn map(&self, code: KeyCode) -> Option<GameAction> {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => Some(GameAction::Quit),
            KeyCode::Char(' ') | KeyCode::Enter => Some(GameAction::Pause),
            KeyCode::Char('f') | KeyCode::Char('x') => Some(GameAction::Fire),
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                Some(GameAction::Move(Direction::Up))
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                Some(GameAction::Move(Direction::Down))
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                Some(GameAction::Move(Direction::Left))
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                Some(GameAction::Move(Direction::Right))
            }
            _ => None,
        }
    }
}
