use std::time::Instant;

use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::{
    appstate::{AppState, PlayingState, Screen},
    components::Grid,
};

#[derive(Debug)]
pub struct MenuState {
    pub cursor_height: u8,
    pub grid_height: usize,
    pub grid_width: usize,
    pub number_of_mines: usize,
}

impl MenuState {
    pub fn move_cursor_down(&mut self) {
        self.cursor_height = (self.cursor_height + 1) % 4
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_height = self.cursor_height.checked_sub(1).unwrap_or(3)
    }

    pub fn increment_value(&mut self) {
        match self.cursor_height {
            0 => self.grid_height += 1,
            1 => self.grid_width += 1,
            2 => self.number_of_mines += 1,
            _ => {}
        }
    }

    pub fn decrement_value(&mut self) {
        match self.cursor_height {
            0 if self.grid_height > 1 => self.grid_height -= 1,
            1 if self.grid_width > 1 => self.grid_width -= 1,
            2 if self.number_of_mines > 1 => self.number_of_mines -= 1,
            _ => {}
        }
    }

    pub fn start(self) -> Result<PlayingState, Error> {
        let grid = Grid::new(self.grid_height, self.grid_width, self.number_of_mines)?;
        Ok(PlayingState {
            grid,
            start_time: Instant::now(),
        })
    }
}

impl Widget for &mut MenuState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // First, bound grid values. This will handle if the terminal has been resized
        self.grid_height = self.grid_height.min(area.height as usize);
        self.grid_width = self.grid_width.min(area.width as usize);
        self.number_of_mines = self
            .number_of_mines
            .min(self.grid_height * self.grid_width - 1)
            .max(1);

        let height_line = if self.grid_height == 1 {
            Line::from(format!("    Grid Height:  {:>4} ▶", self.grid_height)).centered()
        } else if self.grid_height == area.height as usize {
            Line::from(format!("◀   Grid Height:  {:>4}  ", self.grid_height)).centered()
        } else {
            Line::from(format!("◀   Grid Height:  {:>4} ▶", self.grid_height)).centered()
        };

        let width_line = if self.grid_width == 1 {
            Line::from(format!("    Grid Width:   {:>4} ▶", self.grid_width)).centered()
        } else if self.grid_width == area.width as usize {
            Line::from(format!("◀   Grid Width:   {:>4}  ", self.grid_width)).centered()
        } else {
            Line::from(format!("◀   Grid Width:   {:>4} ▶", self.grid_width)).centered()
        };

        let mine_line = if self.number_of_mines == 1 {
            Line::from(format!("  Number of Mines:{:>4} ▶", self.number_of_mines)).centered()
        } else if self.number_of_mines + 1 == self.grid_height * self.grid_width {
            Line::from(format!("◀ Number of Mines:{:>4}  ", self.number_of_mines)).centered()
        } else {
            Line::from(format!("◀ Number of Mines:{:>4} ▶", self.number_of_mines)).centered()
        };

        let start_line = Line::from(" Start ").centered();

        let mut lines = vec![height_line, width_line, mine_line, start_line];
        lines[self.cursor_height as usize] =
            lines[self.cursor_height as usize].clone().fg(Color::Yellow);

        Paragraph::new(lines).left_aligned().render(area, buf);
    }
}

impl Screen for MenuState {
    fn handle_key_event(mut self, key_event: KeyEvent) -> Result<AppState, Error> {
        match key_event.code {
            KeyCode::Char('q') => Ok(AppState::Quit),
            KeyCode::Enter | KeyCode::Char(' ') if self.cursor_height == 3 => {
                Ok(AppState::Playing(self.start()?))
            }
            KeyCode::Down => {
                self.move_cursor_down();
                Ok(AppState::Menu(self))
            }
            KeyCode::Up => {
                self.move_cursor_up();
                Ok(AppState::Menu(self))
            }
            KeyCode::Right => {
                self.increment_value();
                Ok(AppState::Menu(self))
            }
            KeyCode::Left => {
                self.decrement_value();
                Ok(AppState::Menu(self))
            }
            _ => Ok(AppState::Menu(self)),
        }
    }
}
