use std::time::Instant;

use crate::components::Grid;

#[derive(Debug)]
pub struct PlayingState {
    pub grid: Grid,
    pub start_time: Instant,
}
