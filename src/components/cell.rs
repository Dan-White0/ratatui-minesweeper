struct Cell {
    flagged: bool,
}

impl Cell {
    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_toggle_flag() {
        let mut cell = Cell { flagged: false };

        // Flag is off
        assert!(!cell.flagged);

        // Can turn flag on
        cell.toggle_flag();
        assert!(cell.flagged);

        // And off
        cell.toggle_flag();
        assert!(!cell.flagged);
    }
}
