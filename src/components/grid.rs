use crate::components::Cell;

#[derive(Default)]
struct Grid {
    rows: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_column: usize,
}

impl Grid {
    pub fn new(number_of_rows: usize, number_of_columns: usize) -> Self {
        let rows = vec![vec![Cell::default(); number_of_columns]; number_of_rows];
        Grid {
            rows,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_grid() {
        let grid = Grid::new(1, 2);

        let expected_rows = vec![vec![Cell::default(), Cell::default()]];

        assert_eq!(grid.rows, expected_rows);
        assert_eq!(grid.cursor_row, 0);
        assert_eq!(grid.cursor_column, 0);
    }
}
