use anyhow::Error;
use crossterm::event::KeyEvent;

use crate::appstate::AppState;

pub trait Screen {
    fn handle_key_event(self, key_event: KeyEvent) -> Result<AppState, Error>;
}
