use victory_time_rs::Timespan;

use crate::state;
pub mod mock;
pub mod quad;
pub mod runner;
pub mod simulation;
pub mod viz;
pub trait System {
    fn get_settings(&self) -> SystemSettings {
        SystemSettings::default()
    }
    fn init(&mut self, state: &mut state::BasherState);
    fn execute(&mut self, state: &mut state::BasherState, dt: Timespan);
    fn cleanup(&mut self, state: &mut state::BasherState);
}

pub struct SystemSettings {
    pub name: String,
}

impl SystemSettings {
    pub fn new(name: String) -> SystemSettings {
        SystemSettings { name }
    }
}

impl Default for SystemSettings {
    fn default() -> Self {
        SystemSettings {
            name: String::from("Default System"),
        }
    }
}

pub type SystemHandle = Box<dyn System>;
