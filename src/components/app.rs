use std::time::Instant;

use anyhow::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::style::Color,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::components::grid::Grid;

#[derive(Debug)]
pub enum App {
    Menu(MenuState),
    Playing(PlayingState),
    Won(FinishedState),
    Lost(FinishedState),
    Quit,
}

#[derive(Debug)]
pub struct MenuState {
    pub cursor_height: u8,
    pub grid_height: usize,
    pub grid_width: usize,
    pub number_of_mines: usize,
}

#[derive(Debug)]
pub struct PlayingState {
    pub grid: Grid,
    pub start_time: Instant,
}

#[derive(Debug)]
pub struct FinishedState {
    pub grid: Grid,
    pub time_taken_s: u64,
}

impl Default for App {
    fn default() -> Self {
        App::Menu(MenuState {
            cursor_height: 0,
            grid_height: 10,
            grid_width: 20,
            number_of_mines: 10,
        })
    }
}

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(self, area);

        let internal_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        match self {
            App::Menu(state) => {
                frame.render_widget(state, internal_area);
            }
            App::Playing(state) => {
                // draw grid, timer
                frame.render_widget(&state.grid, internal_area);
            }
            App::Won(state) => {
                // draw with green background
                frame.render_widget(&state.grid, internal_area);
            }
            App::Lost(state) => {
                // draw with red background
                frame.render_widget(&state.grid, internal_area);
            }
            App::Quit => unreachable!("App should close before rendering in this state"),
        }
    }

    pub fn handle_events(self) -> Result<Self, Error> {
        if let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            return self.handle_key_event(key_event);
        }

        Ok(self)
    }

    fn handle_key_event(self, key_event: KeyEvent) -> Result<Self, Error> {
        match self {
            App::Menu(mut state) => match key_event.code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Enter if state.cursor_height == 3 => Ok(App::Playing(state.start()?)),
                KeyCode::Down => {
                    state.move_cursor_down();
                    Ok(App::Menu(state))
                }
                KeyCode::Up => {
                    state.move_cursor_up();
                    Ok(App::Menu(state))
                }
                KeyCode::Right => {
                    state.increment_value();
                    Ok(App::Menu(state))
                }
                KeyCode::Left => {
                    state.decrement_value();
                    Ok(App::Menu(state))
                }
                _ => Ok(App::Menu(state)),
            },
            App::Playing(mut state) => match key_event.code {
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Enter => {
                    state.grid.reveal_cell();
                    if state.grid.current_cell().is_mine {
                        Ok(App::Lost(FinishedState {
                            grid: state.grid,
                            time_taken_s: state.start_time.elapsed().as_secs(),
                        }))
                    } else if state.grid.finished() {
                        Ok(App::Won(FinishedState {
                            grid: state.grid,
                            time_taken_s: state.start_time.elapsed().as_secs(),
                        }))
                    } else {
                        Ok(App::Playing(state))
                    }
                }
                KeyCode::Char('f') => {
                    state.grid.flag_cell();
                    Ok(App::Playing(state))
                }
                KeyCode::Down => {
                    state.grid.move_cursor_down();
                    Ok(App::Playing(state))
                }
                KeyCode::Up => {
                    state.grid.move_cursor_up();
                    Ok(App::Playing(state))
                }
                KeyCode::Right => {
                    state.grid.move_cursor_right();
                    Ok(App::Playing(state))
                }
                KeyCode::Left => {
                    state.grid.move_cursor_left();
                    Ok(App::Playing(state))
                }
                _ => {
                    // handle other movement keys, etc.
                    Ok(App::Playing(state))
                }
            },
            state => match key_event.code {
                KeyCode::Char('q') => Ok(Self::Quit),
                _ => Ok(state),
            },
            // Once finished, ignore key events or exit
        }
    }

    pub fn exit(&self) -> bool {
        matches!(self, App::Quit)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Minesweeper! ".bold());

        // compute time + color based on variant
        let (time_elapsed, bg_color) = match self {
            App::Menu(_) => (None, None),
            App::Playing(state) => (Some(state.start_time.elapsed().as_secs()), None),
            App::Won(state) => (Some(state.time_taken_s), Some(Color::Green)),
            App::Lost(state) => (Some(state.time_taken_s), Some(Color::Red)),
            App::Quit => unreachable!("App should quit before rendering!"),
        };

        let mut block;
        if let Some(time_elapsed) = time_elapsed {
            let minutes = time_elapsed / 60;
            let seconds = time_elapsed % 60;
            let timer = Line::from(format!(" {minutes:0>2}:{seconds:0>2} "));

            block = Block::bordered()
                .title(title.centered())
                .title_bottom(timer.centered())
                .border_set(border::THICK);
        } else {
            block = Block::bordered()
                .title(title.centered())
                .border_set(border::THICK);
        }

        if let Some(color) = bg_color {
            block = block.bg(color);
        }

        Paragraph::new(Line::from(""))
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
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

#[cfg(test)]
mod test {
    use crate::components::Cell;

    use super::*;

    #[test]
    fn can_exit() {
        let mut app = App::default();
        assert!(!matches!(app, App::Quit));

        app = app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(matches!(app, App::Quit));
    }

    #[test]
    fn revealing_an_empty_cell_doesnt_change_gamestate_if_there_are_still_unrevealed_empty_cells() {
        // 3x1 grid, mine in third cell
        let grid = Grid::custom(vec![vec![
            Cell {
                neighbouring_mines: 1,
                ..Default::default()
            },
            Cell {
                neighbouring_mines: 1,
                ..Default::default()
            },
            Cell {
                neighbouring_mines: 1,
                is_mine: true,
                ..Default::default()
            },
        ]]);

        let mut app = App::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        });

        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app, App::Playing(_)));
    }

    #[test]
    fn revealing_an_empty_cell_changes_gamestate_to_won_if_there_are_no_unrevealed_empty_cells() {
        // 2x1 grid, mine in second cell
        let grid = Grid::custom(vec![vec![
            Cell {
                neighbouring_mines: 1,
                ..Default::default()
            },
            Cell {
                neighbouring_mines: 1,
                is_mine: true,
                ..Default::default()
            },
        ]]);

        let mut app = App::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        });

        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app, App::Won(_)));
    }

    #[test]
    fn revealing_a_cell_with_a_mine_changes_gamestate_to_lost() {
        // 2x1 grid, mine in second cell
        let grid = Grid::custom(vec![vec![
            Cell {
                neighbouring_mines: 1,
                is_mine: true,
                ..Default::default()
            },
            Cell {
                neighbouring_mines: 1,
                ..Default::default()
            },
        ]]);

        let mut app = App::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        });
        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app, App::Lost(_)));
    }
}
