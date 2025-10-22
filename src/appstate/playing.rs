use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    appstate::{App, LostState, Screen, WonState},
    components::Grid,
};

#[derive(Debug)]
pub struct PlayingState {
    pub grid: Grid,
    pub start_time: Instant,
}

impl Screen for PlayingState {
    fn handle_key_event(mut self, key_event: KeyEvent) -> Result<super::App, anyhow::Error> {
        match key_event.code {
            KeyCode::Char('q') => Ok(App::Quit),
            KeyCode::Enter => {
                self.grid.reveal_cell();
                if self.grid.current_cell().is_mine {
                    Ok(App::Lost(LostState {
                        grid: self.grid,
                        time_taken_s: self.start_time.elapsed().as_secs(),
                    }))
                } else if self.grid.finished() {
                    Ok(App::Won(WonState {
                        grid: self.grid,
                        time_taken_s: self.start_time.elapsed().as_secs(),
                    }))
                } else {
                    Ok(App::Playing(self))
                }
            }
            KeyCode::Char('f') => {
                self.grid.flag_cell();
                Ok(App::Playing(self))
            }
            KeyCode::Down => {
                self.grid.move_cursor_down();
                Ok(App::Playing(self))
            }
            KeyCode::Up => {
                self.grid.move_cursor_up();
                Ok(App::Playing(self))
            }
            KeyCode::Right => {
                self.grid.move_cursor_right();
                Ok(App::Playing(self))
            }
            KeyCode::Left => {
                self.grid.move_cursor_left();
                Ok(App::Playing(self))
            }
            _ => Ok(App::Playing(self)),
        }
    }
}
