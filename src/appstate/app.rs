use std::{io, time::Duration};

use anyhow::{Error, anyhow};
use crossterm::event::{self, Event, EventStream, KeyEvent, KeyEventKind};
use futures::{StreamExt, future::FutureExt, select};
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
use tokio::time::{Interval, interval};

use crate::appstate::{LostState, MenuState, PlayingState, Screen, WonState};

const TIMER_TICK_INTERVAL_MS: Duration = Duration::from_millis(100);

pub struct App {
    app_state: AppState,
    crossterm_events: EventStream,
    interval: Interval,
}

#[derive(Debug)]
pub enum AppState {
    Menu(MenuState),
    Playing(PlayingState),
    Won(WonState),
    Lost(LostState),
    Quit,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_state: AppState::Menu(MenuState {
                cursor_height: 0,
                grid_height: 10,
                grid_width: 20,
                number_of_mines: 10,
            }),
            crossterm_events: EventStream::new(),
            interval: interval(TIMER_TICK_INTERVAL_MS),
        }
    }

    pub fn exit(&self) -> bool {
        matches!(self.app_state, AppState::Quit)
    }
}

impl App {
    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(&self.app_state, area);

        let internal_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        match &mut self.app_state {
            AppState::Menu(state) => {
                frame.render_widget(state, internal_area);
            }
            AppState::Playing(state) => {
                // draw grid, timer
                frame.render_widget(&state.grid, internal_area);
            }
            AppState::Won(state) => {
                // draw with green background
                frame.render_widget(&state.grid, internal_area);
                frame.render_widget(state, internal_area);
            }
            AppState::Lost(state) => {
                // draw with red background
                frame.render_widget(&state.grid, internal_area);
                frame.render_widget(state, internal_area);
            }
            AppState::Quit => unreachable!("App should close before rendering in this state"),
        }
    }

    pub async fn handle_events(mut self) -> Result<Self, Error> {
        select! {
            event = self.crossterm_events.next().fuse() => {
                self.handle_crossterm_event(event)
            },
            _ = self.interval.tick().fuse() => Ok(self),
        }
    }

    fn handle_crossterm_event(
        self,
        event: Option<Result<Event, io::Error>>,
    ) -> Result<Self, Error> {
        match event {
            Some(Ok(event::Event::Key(key))) if key.kind == KeyEventKind::Press => {
                self.handle_key_event(key)
            }
            Some(Err(err)) => Err(anyhow!(err)),
            None => panic!("event stream ended unexpectedly"),
            _ => Ok(self),
        }
    }

    fn handle_key_event(self, key_event: KeyEvent) -> Result<Self, Error> {
        let app_state = match self.app_state {
            AppState::Menu(state) => state.handle_key_event(key_event)?,
            AppState::Playing(state) => state.handle_key_event(key_event)?,
            AppState::Lost(state) => state.handle_key_event(key_event)?,
            AppState::Won(state) => state.handle_key_event(key_event)?,
            AppState::Quit => unreachable!("App should quit before"),
        };
        Ok(App {
            app_state,
            crossterm_events: self.crossterm_events,
            interval: self.interval,
        })
    }
}

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Minesweeper! ".bold());

        // compute time + color based on variant
        let (time_elapsed, bg_color) = match self {
            AppState::Menu(_) => (None, None),
            AppState::Playing(state) => (Some(state.start_time.elapsed().as_secs()), None),
            AppState::Won(state) => (Some(state.time_taken_s), Some(Color::Green)),
            AppState::Lost(state) => (Some(state.time_taken_s), Some(Color::Red)),
            AppState::Quit => unreachable!("App should quit before rendering!"),
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

#[cfg(test)]
impl App {
    fn with_app_state(app_state: AppState) -> Self {
        App {
            app_state,
            crossterm_events: event::EventStream::new(),
            interval: interval(Duration::from_secs(1)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crossterm::event::KeyCode;

    use crate::components::{Cell, Grid};

    use super::*;

    #[tokio::test]
    async fn can_exit() {
        let mut app = App::with_app_state(AppState::Menu(MenuState {
            cursor_height: 0,
            grid_height: 10,
            grid_width: 20,
            number_of_mines: 10,
        }));
        assert!(!matches!(app.app_state, AppState::Quit));

        app = app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(matches!(app.app_state, AppState::Quit));
    }

    #[tokio::test]
    async fn revealing_an_empty_cell_doesnt_change_gamestate_if_there_are_still_unrevealed_empty_cells()
     {
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

        let mut app = App::with_app_state(AppState::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        }));

        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app.app_state, AppState::Playing(_)));
    }

    #[tokio::test]
    async fn revealing_an_empty_cell_changes_gamestate_to_won_if_there_are_no_unrevealed_empty_cells()
     {
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

        let mut app = App::with_app_state(AppState::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        }));

        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app.app_state, AppState::Won(_)));
    }

    #[tokio::test]
    async fn revealing_a_cell_with_a_mine_changes_gamestate_to_lost() {
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

        let mut app = App::with_app_state(AppState::Playing(PlayingState {
            grid,
            start_time: Instant::now(),
        }));
        app = app.handle_key_event(KeyCode::Enter.into()).unwrap();
        assert!(matches!(app.app_state, AppState::Lost(_)));
    }
}
