use anyhow::Error;
use crossterm::event::KeyEvent;

use crate::appstate::App;

pub trait Screen {
    fn handle_key_event(self, key_event: KeyEvent) -> Result<App, Error>;
}
