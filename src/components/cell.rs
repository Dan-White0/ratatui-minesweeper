use ratatui::text::Span;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cell {
    pub flagged: bool,
    pub revealed: bool,
    pub is_mine: bool,
    pub neighbouring_mines: u8,
}

impl Cell {
    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn place_mine(&mut self) {
        self.is_mine = true;
    }

    pub fn as_span(&'_ self) -> Span<'_> {
        Span::from(self.as_string())
    }

    fn as_string(&self) -> String {
        if self.flagged && !self.revealed {
            "F".to_string()
        } else if !self.revealed {
            "#".to_string()
        } else if self.is_mine {
            "X".to_string()
        } else if self.neighbouring_mines > 0 {
            self.neighbouring_mines.to_string()
        } else {
            "_".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test]
    fn can_toggle_flag() {
        let mut cell = Cell::default();

        // Flag is off
        assert!(!cell.flagged);

        // Can turn flag on
        cell.toggle_flag();
        assert!(cell.flagged);

        // And off
        cell.toggle_flag();
        assert!(!cell.flagged);
    }

    #[test]
    fn can_reveal_cell() {
        let mut cell = Cell::default();

        // Cell is not revealed
        assert!(!cell.revealed);

        // Can reveal
        cell.reveal();
        assert!(cell.revealed);

        // Calling again keeps cell revealed
        cell.reveal();
        assert!(cell.revealed);
    }

    #[test]
    fn can_mark_as_mine() {
        let mut cell = Cell::default();

        // Cell is not a mine
        assert!(!cell.is_mine);

        // Can mark as mine
        cell.place_mine();
        assert!(cell.is_mine);
    }

    #[test]
    fn empty_cell_converted_to_expected_string() {
        let mut cell = Cell::default();

        // Unrevealed cell appears as a #
        assert_eq!(cell.as_string(), "#");

        // Revealing the cell makes it appear as an _
        cell.reveal();
        assert_eq!(cell.as_string(), "_");
    }

    #[test]
    fn mined_cell_converted_to_expected_string() {
        let mut cell = Cell {
            is_mine: true,
            ..Default::default()
        };

        // Unrevealed cell appears as a #
        assert_eq!(cell.as_string(), "#");

        // Revealing the cell makes it appear as an X
        cell.reveal();
        assert_eq!(cell.as_string(), "X");
    }

    #[test_case(true, "X" ; "flagged cell is a mine")]
    #[test_case(false, "_" ; "flagged cell is not a mine")]
    fn flagged_cell_converted_to_expected_string(is_mine: bool, revealed_str: &str) {
        let mut cell = Cell {
            flagged: true,
            is_mine,
            ..Default::default()
        };

        // Unrevealed cell appears as an F
        assert_eq!(cell.as_string(), "F");

        cell.reveal();
        assert_eq!(cell.as_string(), revealed_str);
    }

    #[test_case(1, "1" ; "one neighbouring mine")]
    #[test_case(2, "2" ; "two neighbouring mines")]
    #[test_case(3, "3" ; "three neighbouring mines")]
    #[test_case(4, "4" ; "four neighbouring mines")]
    #[test_case(5, "5" ; "five neighbouring mines")]
    #[test_case(6, "6" ; "six neighbouring mines")]
    #[test_case(7, "7" ; "seven neighbouring mines")]
    #[test_case(8, "8" ; "eight neighbouring mines")]
    fn unmined_cell_with_mined_neighbours_converted_to_expected_string(
        neighbouring_mines: u8,
        revealed_str: &str,
    ) {
        let mut cell = Cell {
            neighbouring_mines,
            ..Default::default()
        };

        // Unrevealed cell appears as a #
        assert_eq!(cell.as_string(), "#");

        cell.reveal();
        assert_eq!(cell.as_string(), revealed_str);
    }

    #[test_case(1 ; "one neighbouring mine")]
    #[test_case(2 ; "two neighbouring mines")]
    #[test_case(3 ; "three neighbouring mines")]
    #[test_case(4 ; "four neighbouring mines")]
    #[test_case(5 ; "five neighbouring mines")]
    #[test_case(6 ; "six neighbouring mines")]
    #[test_case(7 ; "seven neighbouring mines")]
    #[test_case(8 ; "eight neighbouring mines")]
    fn mined_cell_with_mined_neighbours_converted_to_expected_string(neighbouring_mines: u8) {
        let mut cell = Cell {
            neighbouring_mines,
            is_mine: true,
            ..Default::default()
        };

        // Unrevealed cell appears as a #
        assert_eq!(cell.as_string(), "#");

        cell.reveal();
        assert_eq!(cell.as_string(), "X");
    }
}
