use std::io;

use anyhow::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::components::{gamestate::GameState, grid::Grid};

#[derive(Debug)]
pub struct App {
    exit: bool,
    grid: Grid,
    gamestate: GameState,
}

impl App {
    pub fn new(
        grid_height: usize,
        grid_width: usize,
        number_of_mines: usize,
    ) -> Result<Self, Error> {
        Ok(App {
            exit: false,
            grid: Grid::new(grid_height, grid_width, number_of_mines)?,
            gamestate: GameState::Playing,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(self, area);

        let grid_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };
        frame.render_widget(&self.grid, grid_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => {
                self.grid.move_cursor_down();
            }
            KeyCode::Up => {
                self.grid.move_cursor_up();
            }
            KeyCode::Right => {
                self.grid.move_cursor_right();
            }
            KeyCode::Left => {
                self.grid.move_cursor_left();
            }
            KeyCode::Enter => {
                self.reveal_cell();
            }
            KeyCode::Char('f') => {
                self.grid.flag_cell();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn reveal_cell(&mut self) {
        self.grid.reveal_cell();
        if self.grid.current_cell().is_mine {
            self.gamestate = GameState::Lost;
        } else if self.grid.finished() {
            self.gamestate = GameState::Won
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Minesweeper! ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new(Line::from(""))
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod test {
    use crate::components::Cell;

    use super::*;

    #[test]
    fn can_exit() {
        let mut app = App::new(1, 1, 1).unwrap();
        assert!(!app.exit);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
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

        let mut app = App {
            exit: false,
            gamestate: GameState::Playing,
            grid,
        };
        assert!(!app.exit);

        app.handle_key_event(KeyCode::Enter.into());
        assert_eq!(app.gamestate, GameState::Playing);
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

        let mut app = App {
            exit: false,
            gamestate: GameState::Playing,
            grid,
        };
        assert!(!app.exit);

        app.handle_key_event(KeyCode::Enter.into());
        assert_eq!(app.gamestate, GameState::Won);
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

        let mut app = App {
            exit: false,
            gamestate: GameState::Playing,
            grid,
        };
        assert!(!app.exit);

        app.handle_key_event(KeyCode::Enter.into());
        assert_eq!(app.gamestate, GameState::Lost);
    }
}
