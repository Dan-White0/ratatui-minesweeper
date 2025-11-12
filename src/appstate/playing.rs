use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    appstate::{AppState, LostState, Screen, WonState},
    components::Grid,
};

#[derive(Debug)]
pub struct PlayingState {
    pub grid: Grid,
    pub start_time: Instant,
}

impl Screen for PlayingState {
    fn handle_key_event(mut self, key_event: KeyEvent) -> Result<super::AppState, anyhow::Error> {
        match key_event.code {
            KeyCode::Char('q') => Ok(AppState::Quit),
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.grid.reveal_cell();
                if self.grid.current_cell().is_mine {
                    Ok(AppState::Lost(LostState {
                        grid: self.grid,
                        time_taken_s: self.start_time.elapsed().as_secs(),
                    }))
                } else if self.grid.finished() {
                    Ok(AppState::Won(WonState {
                        grid: self.grid,
                        time_taken_s: self.start_time.elapsed().as_secs(),
                    }))
                } else {
                    Ok(AppState::Playing(self))
                }
            }
            KeyCode::Char('f') => {
                self.grid.flag_cell();
                Ok(AppState::Playing(self))
            }
            KeyCode::Down => {
                self.grid.move_cursor_down();
                Ok(AppState::Playing(self))
            }
            KeyCode::Up => {
                self.grid.move_cursor_up();
                Ok(AppState::Playing(self))
            }
            KeyCode::Right => {
                self.grid.move_cursor_right();
                Ok(AppState::Playing(self))
            }
            KeyCode::Left => {
                self.grid.move_cursor_left();
                Ok(AppState::Playing(self))
            }
            _ => Ok(AppState::Playing(self)),
        }
    }
}
