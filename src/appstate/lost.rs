use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    appstate::{App, Screen},
    components::Grid,
};

#[derive(Debug)]
pub struct LostState {
    pub grid: Grid,
    pub time_taken_s: u64,
}

impl Screen for LostState {
    fn handle_key_event(self, key_event: KeyEvent) -> Result<App, Error> {
        match key_event.code {
            KeyCode::Char('q') => Ok(App::Quit),
            _ => Ok(App::Lost(self)),
        }
    }
}
