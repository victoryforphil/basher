use crate::state;
pub mod mock;
pub mod quad;
pub mod runner;
pub trait System {
    fn init(&mut self, state: &mut state::BasherState);
    fn execute(&mut self, state: &mut state::BasherState);
    fn cleanup(&mut self, state: &mut state::BasherState);
}

pub type SystemHandle = Box<dyn System>;
