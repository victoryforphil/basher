use quad_state::QuadState;

use crate::types::QuadPose;
pub mod quad_state;
#[derive(Debug)]
pub struct BasherState {
    pub quad: QuadState,
}

impl BasherState {
    pub fn new() -> BasherState {
        BasherState {
            quad: QuadState::new(),
        }
    }
    pub fn with_quad(mut self, quad: QuadState) -> BasherState {
        self.quad = quad;
        self
    }
}
