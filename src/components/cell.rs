use ratatui::text::Span;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cell {
    flagged: bool,
    revealed: bool,
    pub is_mine: bool,
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
        Span::from(self.as_str())
    }

    fn as_str(&self) -> &str {
        if !self.revealed {
            "#"
        } else if self.is_mine {
            "X"
        } else {
            "_"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(cell.as_str(), "#");

        // Revealing the cell makes it appear as an _
        cell.reveal();
        assert_eq!(cell.as_str(), "_");
    }

    #[test]
    fn mined_cell_converted_to_expected_string() {
        let mut cell = Cell {
            is_mine: true,
            ..Default::default()
        };

        // Unrevealed cell appears as a #
        assert_eq!(cell.as_str(), "#");

        // Revealing the cell makes it appear as an X
        cell.reveal();
        assert_eq!(cell.as_str(), "X");
    }
}
