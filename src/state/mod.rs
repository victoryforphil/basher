use quad_state::QuadState;
use rerun::Time;
use victory_time_rs::Timepoint;

use crate::types::QuadPose;
pub mod quad_state;
#[derive(Debug)]
pub struct BasherState {
    pub quad: QuadState,
    pub current_time: Timepoint,
}

impl BasherState {
    pub fn new() -> BasherState {
        BasherState {
            quad: QuadState::new(),
            current_time: Timepoint::zero(),
        }
    }
    pub fn with_quad(mut self, quad: QuadState) -> BasherState {
        self.quad = quad;
        self
    }
}
