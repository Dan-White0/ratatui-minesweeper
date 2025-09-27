use rand::seq::index::sample;

use crate::components::Cell;

#[derive(Default)]
struct Grid {
    rows: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_column: usize,
}

impl Grid {
    pub fn new(number_of_rows: usize, number_of_columns: usize, number_of_bombs: usize) -> Self {
        let mut rows = vec![vec![Cell::default(); number_of_columns]; number_of_rows];

        let total_cells = number_of_rows * number_of_columns;
        let mut rng = rand::rng();

        for index in sample(&mut rng, total_cells, number_of_bombs).iter() {
            let row_index = index / number_of_rows;
            let column_index = index % number_of_columns;

            rows[row_index][column_index].place_bomb();
        }

        Grid {
            rows,
            ..Default::default()
        }
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
    fn can_create_grid(grid_height: usize, grid_width: usize) {
        let grid = Grid::new(grid_height, grid_width, 0);

        assert_eq!(grid.rows.len(), grid_height);
        assert_eq!(grid.rows[0].len(), grid_width);
        assert_eq!(grid.cursor_row, 0);
        assert_eq!(grid.cursor_column, 0);
    }

    #[test_case(0 ; "no bombs")]
    #[test_case(1 ; "one bomb")]
    #[test_case(10 ; "many bombs")]
    fn can_create_grid_with_expected_number_of_bombs(expected_number_of_bombs: usize) {
        let grid = Grid::new(5, 5, expected_number_of_bombs);

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
