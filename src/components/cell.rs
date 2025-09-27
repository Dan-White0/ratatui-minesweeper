struct Cell {
    flagged: bool,
    revealed: bool,
}

impl Cell {
    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_toggle_flag() {
        let mut cell = Cell {
            flagged: false,
            revealed: false,
        };

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
        let mut cell = Cell {
            flagged: false,
            revealed: false,
        };

        // Cell is not revealed
        assert!(!cell.revealed);

        // Can reveal
        cell.reveal();
        assert!(cell.revealed);

        // Calling again keeps cell revealed
        cell.reveal();
        assert!(cell.revealed);
    }
}
