use std::time::Instant;

use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    crossterm::style::Color,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

const WIN_SCREEN_POPUP_WIDTH: u16 = 30;
const WIN_SCREEN_POPUP_HEIGHT: u16 = 10;

use crate::{
    appstate::{AppState, PlayingState, Screen},
    components::{Grid, center},
};

#[derive(Debug)]
pub struct WonState {
    pub grid: Grid,
    pub time_taken_s: u64,
}

impl Screen for WonState {
    fn handle_key_event(self, key_event: KeyEvent) -> Result<AppState, Error> {
        match key_event.code {
            KeyCode::Char('q') => Ok(AppState::Quit),
            KeyCode::Char('r') => {
                let grid = Grid::new(self.grid.rows(), self.grid.columns(), self.grid.mines())?;
                Ok(AppState::Playing(PlayingState {
                    grid,
                    start_time: Instant::now(),
                }))
            }
            _ => Ok(AppState::Won(self)),
        }
    }
}

impl Widget for &mut WonState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_set(border::DOUBLE);

        let minutes = self.time_taken_s / 60;
        let seconds = self.time_taken_s % 60;
        let timer = Line::from(format!(" {minutes:0>2}:{seconds:0>2} "));

        let win_text = vec![
            Line::from(""),
            Line::from("You win!").centered(),
            Line::from(""),
            Line::from(format!("Time taken: {timer}")).centered(),
            Line::from(""),
            Line::from("R - Restart  Q - Quit").centered(),
        ];

        let area = center(
            area,
            Constraint::from(WIN_SCREEN_POPUP_WIDTH),
            Constraint::from(WIN_SCREEN_POPUP_HEIGHT),
        );

        Clear.render(area, buf);

        Paragraph::new(win_text)
            .block(block)
            .bg(Color::Green)
            .render(area, buf);
    }
}
