use commander::CommanderState;
use quad_state::QuadState;
use victory_time_rs::Timepoint;

pub mod commander;
pub mod quad_state;
pub mod waypoint;
#[derive(Debug)]
pub struct BasherState {
    pub quad: QuadState,
    pub current_time: Timepoint,
    pub commander: CommanderState,
}

impl BasherState {
    pub fn new() -> BasherState {
        BasherState {
            quad: QuadState::new(),
            current_time: Timepoint::zero(),
            commander: CommanderState::new(),
        }
    }
    pub fn with_quad(mut self, quad: QuadState) -> BasherState {
        self.quad = quad;
        self
    }
}
