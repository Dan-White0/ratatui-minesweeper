use anyhow::{Error, anyhow};
use rand::seq::index::sample;

use crate::components::Cell;

#[derive(Debug, Default)]
struct Grid {
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
                "Attempting to create a grid with more bombs than cells: {}x{} grid ({} cells), but tried to insert {} bombs",
                number_of_rows,
                number_of_columns,
                total_cells,
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
            rows,
            ..Default::default()
        })
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
        let grid = Grid::new(number_of_rows, number_of_columns, 1);

        assert!(grid.is_ok());

        let grid = grid.unwrap();

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
            "Attempting to create a grid with more bombs than cells: 2x3 grid (6 cells), but tried to insert 7 bombs"
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
        let grid = Grid::new(5, 5, expected_number_of_bombs);

        assert!(grid.is_ok());

        let grid = grid.unwrap();

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
}
