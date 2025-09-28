use anyhow::{Error, anyhow};
use rand::seq::index::sample;

use crate::components::Cell;

#[derive(Debug, Default)]
struct Grid {
    number_of_rows: usize,
    number_of_columns: usize,
    rows: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_column: usize,
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

        for index in sample(&mut rng, total_cells, number_of_mines).iter() {
            let row_index = index / number_of_columns;
            let column_index = index % number_of_columns;

            rows[row_index][column_index].place_mine();
        }

        Ok(Grid {
            number_of_rows,
            number_of_columns,
            rows,
            cursor_row: 0,
            cursor_column: 0,
        })
    }

    fn move_cursor_right(&mut self) {
        self.cursor_column = (self.cursor_column + 1) % self.number_of_columns
    }

    fn move_cursor_left(&mut self) {
        self.cursor_column = self
            .cursor_column
            .checked_sub(1)
            .unwrap_or_else(|| self.number_of_columns - 1)
            % self.number_of_columns
    }

    fn move_cursor_down(&mut self) {
        self.cursor_row = (self.cursor_row + 1) % self.number_of_rows
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(1, 1 ; "tiny grid")]
    #[test_case(1 , 100; "wide grid")]
    #[test_case(100 , 1; "tall grid")]
    #[test_case(100 , 100; "large grid")]
    fn can_create_grid(number_of_rows: usize, number_of_columns: usize) {
        let grid = Grid::new(number_of_rows, number_of_columns, 1).unwrap();

        assert_eq!(grid.rows.len(), number_of_rows);
        assert_eq!(grid.rows[0].len(), number_of_columns);
        assert_eq!(grid.cursor_row, 0);
        assert_eq!(grid.cursor_column, 0);
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
}
