#[derive(Default)]
pub struct Cell {
    flagged: bool,
    revealed: bool,
    is_bomb: bool,
}

impl Cell {
    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn place_bomb(&mut self) {
        self.is_bomb = true;
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
    fn can_mark_as_bomb() {
        let mut cell = Cell::default();

        // Cell is not a bomb
        assert!(!cell.is_bomb);

        // Can mark as bomb
        cell.place_bomb();
        assert!(cell.is_bomb);
    }
}
