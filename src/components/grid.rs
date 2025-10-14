use anyhow::{Error, anyhow};
use rand::seq::index::sample;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

use crate::components::{Cell, layout::center};

#[derive(Debug, Default)]
pub struct Grid {
    number_of_rows: usize,
    number_of_columns: usize,
    rows: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_column: usize,
    remaining_empty_cells: usize,
}

impl Grid {
    pub fn new(
        number_of_rows: usize,
        number_of_columns: usize,
        number_of_mines: usize,
    ) -> Result<Self, Error> {
        if number_of_mines == 0 {
            return Err(anyhow!("Cannot create a grid with no mines"));
        }

        let total_cells = number_of_rows * number_of_columns;

        if number_of_mines > total_cells {
            return Err(anyhow!(
                "Attempting to create a grid with more mines than cells: {} cells ({}x{} grid), but tried to insert {} mines",
                total_cells,
                number_of_rows,
                number_of_columns,
                number_of_mines
            ));
        }

        let mut rows = vec![vec![Cell::default(); number_of_columns]; number_of_rows];

        let mut rng = rand::rng();

        for mined_cell_index in sample(&mut rng, total_cells, number_of_mines).iter() {
            Grid::place_mine(&mut rows, number_of_columns, mined_cell_index);
        }

        Ok(Grid {
            number_of_rows,
            number_of_columns,
            rows,
            cursor_row: 0,
            cursor_column: 0,
            remaining_empty_cells: total_cells - number_of_mines,
        })
    }

    fn place_mine(rows: &mut [Vec<Cell>], number_of_columns: usize, mined_cell_index: usize) {
        let row_index = mined_cell_index / number_of_columns;
        let column_index = mined_cell_index % number_of_columns;

        rows[row_index][column_index].place_mine();

        for row in rows
            .iter_mut()
            .take(row_index + 2)
            .skip(row_index.saturating_sub(1))
        {
            for cell in row
                .iter_mut()
                .take(column_index + 2)
                .skip(column_index.saturating_sub(1))
            {
                cell.neighbouring_mines += 1;
            }
        }
    }

    pub fn current_cell(&self) -> &Cell {
        &self.rows[self.cursor_row][self.cursor_column]
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_column = (self.cursor_column + 1) % self.number_of_columns
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_column = self
            .cursor_column
            .checked_sub(1)
            .unwrap_or_else(|| self.number_of_columns - 1)
            % self.number_of_columns
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_row = (self.cursor_row + 1) % self.number_of_rows
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_row = self
            .cursor_row
            .checked_sub(1)
            .unwrap_or_else(|| self.number_of_rows - 1)
            % self.number_of_rows
    }

    pub fn reveal_cell(&mut self) {
        self._reveal_cell(self.cursor_row, self.cursor_column);
    }

    fn _reveal_cell(&mut self, cell_row: usize, cell_column: usize) {
        let cell = &mut self.rows[cell_row][cell_column];
        if cell.revealed {
            return;
        }
        cell.reveal();
        self.remaining_empty_cells -= 1;
        if cell.neighbouring_mines == 0 {
            for i in cell_row.saturating_sub(1)..(cell_row + 2).min(self.number_of_rows) {
                for j in
                    cell_column.saturating_sub(1)..(cell_column + 2).min(self.number_of_columns)
                {
                    self._reveal_cell(i, j);
                }
            }
        }
    }

    pub fn flag_cell(&mut self) {
        self.rows[self.cursor_row][self.cursor_column].toggle_flag();
    }

    pub fn finished(&self) -> bool {
        self.remaining_empty_cells == 0
    }
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows: Vec<Line<'_>> = self
            .rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                let mut line = Line::from(
                    row.iter()
                        .enumerate()
                        .map(|(column_index, cell)| {
                            let mut span = cell.as_span();

                            if column_index == self.cursor_column {
                                span = span.bg(Color::DarkGray);
                            }

                            span
                        })
                        .collect::<Vec<Span>>(),
                );

                if row_index == self.cursor_row {
                    line = line.bg(Color::DarkGray);
                }

                line
            })
            .collect();

        let area = center(
            area,
            Constraint::Length(self.number_of_columns as u16),
            Constraint::Length(self.number_of_rows as u16),
        );

        Paragraph::new(Text::from(rows)).render(area, buf);
    }
}

#[cfg(test)]
impl Grid {
    pub fn custom(rows: Vec<Vec<Cell>>) -> Self {
        let number_of_rows = rows.len();
        let number_of_columns = rows[0].len();
        let mut number_of_mines = 0;
        for row in &rows {
            for cell in row {
                if cell.is_mine {
                    number_of_mines += 1;
                }
            }
        }

        Self {
            rows: rows,
            number_of_rows,
            number_of_columns,
            cursor_row: 0,
            cursor_column: 0,
            remaining_empty_cells: (number_of_rows * number_of_columns) - number_of_mines,
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Style;
    use test_case::test_case;

    use super::*;

    #[test_case(1, 1, 1, 0 ; "tiny grid")]
    #[test_case(1 , 100, 5, 95; "wide grid")]
    #[test_case(100 , 1, 10, 90; "tall grid")]
    #[test_case(100 , 100, 200, 9800; "large grid")]
    fn can_create_grid(
        number_of_rows: usize,
        number_of_columns: usize,
        number_of_mines: usize,
        number_of_empty_cells: usize,
    ) {
        let grid = Grid::new(number_of_rows, number_of_columns, number_of_mines).unwrap();

        assert_eq!(grid.rows.len(), number_of_rows);
        assert_eq!(grid.rows[0].len(), number_of_columns);
        assert_eq!(grid.cursor_row, 0);
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.remaining_empty_cells, number_of_empty_cells);
    }

    #[test]
    fn return_err_if_trying_to_create_grid_with_more_mines_than_cells() {
        let error = Grid::new(2, 3, 7).unwrap_err();

        // Check top error or context
        assert_eq!(
            format!("{}", error),
            "Attempting to create a grid with more mines than cells: 6 cells (2x3 grid), but tried to insert 7 mines"
        );
    }

    #[test]
    fn return_err_if_trying_to_create_grid_with_no_mines() {
        let error = Grid::new(2, 3, 0).unwrap_err();

        // Check top error or context
        assert_eq!(format!("{}", error), "Cannot create a grid with no mines");
    }

    #[test_case(1 ; "one mine")]
    #[test_case(10 ; "many mines")]
    fn can_create_grid_with_expected_number_of_mines(expected_number_of_mines: usize) {
        let grid = Grid::new(5, 5, expected_number_of_mines).unwrap();

        let mut number_of_mines = 0;

        for row in grid.rows {
            for cell in row {
                if cell.is_mine {
                    number_of_mines += 1;
                }
            }
        }

        assert_eq!(expected_number_of_mines, number_of_mines)
    }

    #[test]
    fn can_move_cursor_right() {
        let mut grid = Grid::new(2, 3, 1).unwrap();

        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor right
        grid.move_cursor_right();
        assert_eq!(grid.cursor_column, 1);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor right again
        grid.move_cursor_right();
        assert_eq!(grid.cursor_column, 2);
        assert_eq!(grid.cursor_row, 0);

        // Moving again will wrap around to the start
        grid.move_cursor_right();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);
    }

    #[test]
    fn can_move_cursor_left() {
        let mut grid = Grid::new(2, 3, 1).unwrap();

        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor left and it wraps around
        grid.move_cursor_left();
        assert_eq!(grid.cursor_column, 2);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor left again
        grid.move_cursor_left();
        assert_eq!(grid.cursor_column, 1);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor left again and it's back to the start
        grid.move_cursor_left();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);
    }

    #[test]
    fn can_move_cursor_down() {
        let mut grid = Grid::new(3, 3, 1).unwrap();

        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor down
        grid.move_cursor_down();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 1);

        // Can move cursor down again
        grid.move_cursor_down();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 2);

        // Can move cursor down again and it wraps back to the top
        grid.move_cursor_down();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);
    }

    #[test]
    fn can_move_cursor_up() {
        let mut grid = Grid::new(3, 3, 1).unwrap();

        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);

        // Can move cursor up and it wraps around
        grid.move_cursor_up();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 2);

        // Can move cursor up again
        grid.move_cursor_up();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 1);

        // Can move cursor up again and it's back to the start
        grid.move_cursor_up();
        assert_eq!(grid.cursor_column, 0);
        assert_eq!(grid.cursor_row, 0);
    }

    #[test]
    fn unrevealed_grid_rendered_as_expected() {
        let grid = Grid::new(2, 3, 1).unwrap();
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));

        grid.render(buf.area, &mut buf);

        #[rustfmt::skip]  // It's nice to have each element of the vec to be on a separate line, like in the terminal
        let mut expected = Buffer::with_lines(vec![
            "###",
            "###",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);

        expected.set_style(Rect::new(0, 0, 3, 1), selected_index_style);
        expected.set_style(Rect::new(0, 1, 1, 1), selected_index_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn moving_cusor_updates_selected_row_and_column() {
        let mut grid = Grid::new(2, 3, 1).unwrap();
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));

        grid.render(buf.area, &mut buf);

        // Grid first formatted as expected
        #[rustfmt::skip]
        let mut expected = Buffer::with_lines(vec![
            "###",
            "###",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);
        expected.set_style(Rect::new(0, 0, 3, 1), selected_index_style);
        expected.set_style(Rect::new(0, 1, 1, 1), selected_index_style);

        assert_eq!(buf, expected);

        // Moving the cursor right shifts the background colour to the second column
        grid.move_cursor_right();

        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));
        grid.render(buf.area, &mut buf);

        #[rustfmt::skip]
        let mut expected = Buffer::with_lines(vec![
            "###",
            "###",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);
        expected.set_style(Rect::new(0, 0, 3, 1), selected_index_style);
        expected.set_style(Rect::new(1, 1, 1, 1), selected_index_style);

        assert_eq!(buf, expected);

        // Moving the cursor down shifts the background colour to the second row
        grid.move_cursor_down();

        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));
        grid.render(buf.area, &mut buf);

        #[rustfmt::skip]
        let mut expected = Buffer::with_lines(vec![
            "###",
            "###",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);
        expected.set_style(Rect::new(1, 0, 1, 1), selected_index_style);
        expected.set_style(Rect::new(0, 1, 3, 1), selected_index_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn mining_a_cell_updates_neighbouring_cells_to_have_a_mined_neighbour() {
        // 3x3 grid
        let mut cell_vec = vec![
            vec![Cell::default(), Cell::default(), Cell::default()],
            vec![Cell::default(), Cell::default(), Cell::default()],
            vec![Cell::default(), Cell::default(), Cell::default()],
        ];

        // No cells are mined at the start
        assert!(!cell_vec[0].iter().any(|cell: &Cell| cell.is_mine));
        assert!(!cell_vec[1].iter().any(|cell: &Cell| cell.is_mine));
        assert!(!cell_vec[2].iter().any(|cell: &Cell| cell.is_mine));

        // Place mine in the top left corner
        Grid::place_mine(&mut cell_vec, 3, 0);

        // Only top left cell is mined
        assert!(cell_vec[0][0].is_mine); // is mined
        assert!(!cell_vec[0][1].is_mine); // other cells on row are not mined
        assert!(!cell_vec[0][2].is_mine);
        assert!(!cell_vec[1].iter().any(|cell: &Cell| cell.is_mine));
        assert!(!cell_vec[2].iter().any(|cell: &Cell| cell.is_mine));

        // Cells neighbouring top left cell have their amount of neighbouring mines updated
        assert_eq!(cell_vec[0][1].neighbouring_mines, 1);
        assert_eq!(cell_vec[1][0].neighbouring_mines, 1);
        assert_eq!(cell_vec[1][1].neighbouring_mines, 1);

        // The other cells are unaffected
        assert_eq!(cell_vec[0][2].neighbouring_mines, 0);
        assert_eq!(cell_vec[1][2].neighbouring_mines, 0);
        assert_eq!(cell_vec[2][0].neighbouring_mines, 0);
        assert_eq!(cell_vec[2][1].neighbouring_mines, 0);
        assert_eq!(cell_vec[2][2].neighbouring_mines, 0);

        // Place mine in the bottom right corner
        Grid::place_mine(&mut cell_vec, 3, 8);

        // Both top left and bottom right cell are mined
        assert!(cell_vec[0][0].is_mine); // is mined
        assert!(!cell_vec[0][1].is_mine); // other cells on row are not mined
        assert!(!cell_vec[0][2].is_mine);
        assert!(!cell_vec[1].iter().any(|cell: &Cell| cell.is_mine));
        assert!(!cell_vec[2][0].is_mine); // other cells on row are not mined
        assert!(!cell_vec[2][1].is_mine);
        assert!(cell_vec[2][2].is_mine); // is mined

        // Cells neighbouring mines updated
        assert_eq!(cell_vec[0][1].neighbouring_mines, 1);
        assert_eq!(cell_vec[1][0].neighbouring_mines, 1);
        assert_eq!(cell_vec[1][1].neighbouring_mines, 2); // neighbouring both mines
        assert_eq!(cell_vec[1][2].neighbouring_mines, 1);
        assert_eq!(cell_vec[2][1].neighbouring_mines, 1);

        // The other cells are unaffected
        assert_eq!(cell_vec[0][2].neighbouring_mines, 0);
        assert_eq!(cell_vec[2][0].neighbouring_mines, 0);
    }

    #[test]
    fn reveling_cell_with_no_neighbouring_mines_reveals_further_cells() {
        // 3x2 grid with a single mine in the top left cell
        let mut grid = Grid::custom(vec![
            vec![
                Cell::default(),
                Cell {
                    neighbouring_mines: 1,
                    ..Default::default()
                },
                Cell {
                    neighbouring_mines: 1,
                    is_mine: true,
                    ..Default::default()
                },
            ],
            vec![
                Cell::default(),
                Cell {
                    neighbouring_mines: 1,
                    ..Default::default()
                },
                Cell {
                    neighbouring_mines: 1,
                    ..Default::default()
                },
            ],
        ]);
        // Starts as 3x2 grid with 1 mine, so 5 empty unrevealed cells
        assert_eq!(grid.remaining_empty_cells, 5);
        assert!(!grid.finished());

        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));

        grid.render(buf.area, &mut buf);

        // Grid first formatted as expected
        #[rustfmt::skip]
        let mut expected = Buffer::with_lines(vec![
            "###",
            "###",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);
        expected.set_style(Rect::new(0, 0, 3, 1), selected_index_style);
        expected.set_style(Rect::new(0, 1, 1, 1), selected_index_style);

        assert_eq!(buf, expected);

        // Revealing the top left cell will reveal more neighbouring cells, as it doesn't neighbour a mine
        grid.reveal_cell();

        assert_eq!(grid.remaining_empty_cells, 1);
        assert!(!grid.finished());

        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));
        grid.render(buf.area, &mut buf);

        #[rustfmt::skip]
        let mut expected = Buffer::with_lines(vec![
            "_1#",
            "_1#",
        ]);

        let selected_index_style = Style::new().bg(Color::DarkGray);
        expected.set_style(Rect::new(0, 0, 3, 1), selected_index_style);
        expected.set_style(Rect::new(0, 1, 1, 1), selected_index_style);

        assert_eq!(buf, expected);
    }
}
