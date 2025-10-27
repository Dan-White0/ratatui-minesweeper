use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{
    appstate::{App, Screen},
    components::{Grid, center},
};

const LOSE_SCREEN_POPUP_WIDTH: u16 = 30;
const LOSE_SCREEN_POPUP_HEIGHT: u16 = 8;

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

impl Widget for &LostState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_set(border::DOUBLE);

        let win_text = vec![
            Line::from(""),
            Line::from("You lose!").centered(),
            Line::from(""),
            Line::from(format!(
                "Remaining cells: {}",
                self.grid.remaining_empty_cells
            ))
            .centered(),
        ];

        let area = center(
            area,
            Constraint::from(LOSE_SCREEN_POPUP_WIDTH),
            Constraint::from(LOSE_SCREEN_POPUP_HEIGHT),
        );

        Clear.render(area, buf);

        Paragraph::new(win_text)
            .block(block)
            .bg(Color::Red)
            .render(area, buf);
    }
}
