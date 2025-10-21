use crate::components::Grid;

#[derive(Debug)]
pub struct FinishedState {
    pub grid: Grid,
    pub time_taken_s: u64,
}
