use std::time::Instant;

use anyhow::Error;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::{appstate::PlayingState, components::Grid};

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
            2 if self.number_of_mines < self.grid_height * self.grid_width => {
                self.number_of_mines += 1
            }
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

impl Widget for &MenuState {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        Paragraph::new(lines)
            .left_aligned()
            // .block(block)
            .render(area, buf);
    }
}
