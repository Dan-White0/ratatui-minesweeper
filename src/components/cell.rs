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

    use rstest::rstest;

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

    #[rstest]
    #[case::flagged_cell_is_a_mine(true, "X")] // flagged cell is a mine
    #[case::flagged_cell_is_not_a_mine(false, "_")] // flagged cell is a mine
    fn flagged_cell_converted_to_expected_string(
        #[case] is_mine: bool,
        #[case] revealed_str: &str,
    ) {
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

    #[rstest]
    #[case::one_neighbouring_mine(1, "1")]
    #[case::two_neighbouring_mines(2, "2")]
    #[case::three_neighbouring_mines(3, "3")]
    #[case::four_neighbouring_mines(4, "4")]
    #[case::five_neighbouring_mines(5, "5")]
    #[case::six_neighbouring_mines(6, "6")]
    #[case::seven_neighbouring_mines(7, "7")]
    #[case::eight_neighbouring_mines(8, "8")]
    fn unmined_cell_with_mined_neighbours_converted_to_expected_string(
        #[case] neighbouring_mines: u8,
        #[case] revealed_str: &str,
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
    #[rstest]
    #[case::one_neighbouring_mine(1)]
    #[case::two_neighbouring_mines(2)]
    #[case::three_neighbouring_mines(3)]
    #[case::four_neighbouring_mines(4)]
    #[case::five_neighbouring_mines(5)]
    #[case::six_neighbouring_mines(6)]
    #[case::seven_neighbouring_mines(7)]
    #[case::eight_neighbouring_mines(8)]
    fn mined_cell_with_mined_neighbours_converted_to_expected_string(
        #[case] neighbouring_mines: u8,
    ) {
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
