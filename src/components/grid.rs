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
        number_of_bombs: usize,
    ) -> Result<Self, Error> {
        if number_of_bombs == 0 {
            return Err(anyhow!("Cannot create a grid with no bombs"));
        }

        let total_cells = number_of_rows * number_of_columns;

        if number_of_bombs > total_cells {
            return Err(anyhow!(
                "Attempting to create a grid with more bombs than cells: {} cells ({}x{} grid), but tried to insert {} bombs",
                total_cells,
                number_of_rows,
                number_of_columns,
                number_of_bombs
            ));
        }

        let mut rows = vec![vec![Cell::default(); number_of_columns]; number_of_rows];

        let mut rng = rand::rng();

        for index in sample(&mut rng, total_cells, number_of_bombs).iter() {
            let row_index = index / number_of_columns;
            let column_index = index % number_of_columns;

            rows[row_index][column_index].place_bomb();
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
    fn return_err_if_trying_to_create_grid_with_more_bombs_than_cells() {
        let error = Grid::new(2, 3, 7).unwrap_err();

        // Check top error or context
        assert_eq!(
            format!("{}", error),
            "Attempting to create a grid with more bombs than cells: 6 cells (2x3 grid), but tried to insert 7 bombs"
        );
    }

    #[test]
    fn return_err_if_trying_to_create_grid_with_no_bombs() {
        let error = Grid::new(2, 3, 0).unwrap_err();

        // Check top error or context
        assert_eq!(format!("{}", error), "Cannot create a grid with no bombs");
    }

    #[test_case(1 ; "one bomb")]
    #[test_case(10 ; "many bombs")]
    fn can_create_grid_with_expected_number_of_bombs(expected_number_of_bombs: usize) {
        let grid = Grid::new(5, 5, expected_number_of_bombs).unwrap();

        let mut number_of_bombs = 0;

        for row in grid.rows {
            for cell in row {
                if cell.is_bomb {
                    number_of_bombs += 1;
                }
            }
        }

        assert_eq!(expected_number_of_bombs, number_of_bombs)
    }

    #[test]
    fn can_move_cursor_horizontally() {
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
}
