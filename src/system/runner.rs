use std::time::{Duration, Instant};

use log::info;
use rerun::Time;
use victory_time_rs::{Timepoint, Timespan};

use crate::state::BasherState;

use super::SystemHandle;

pub struct BasherSysRunner {
    pub systems: Vec<SystemHandle>,
    pub state: BasherState,
    pub end_time: Timepoint,
}

impl BasherSysRunner {
    pub fn new() -> BasherSysRunner {
        BasherSysRunner {
            systems: Vec::new(),
            state: BasherState::new(),
            end_time: Timepoint::zero(),
        }
    }

    pub fn add_system(&mut self, system: SystemHandle) {
        info!("Adding System: {}", system.get_settings().name);
        self.systems.push(system);
    }

    pub fn run(&mut self, duration: Timespan) {
        self.end_time = self.state.current_time.clone() + duration;
        for system in self.systems.iter_mut() {
            system.init(&mut self.state);
        }
        let dt = Timespan::new_hz(100.0);

        while &self.state.current_time < &self.end_time {
            for system in self.systems.iter_mut() {
                system.execute(&mut self.state, dt.clone());
            }
            self.state.current_time = self.state.current_time.clone() + dt.clone();
        }

        for system in self.systems.iter_mut() {
            system.cleanup(&mut self.state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::mock::MockSystem;

    #[test]
    fn test_basher_sys_runner() {
        let mut runner = BasherSysRunner::new();
        let system_a = Box::new(MockSystem::new());
        let system_b = Box::new(MockSystem::new());
        runner.add_system(system_a);
        runner.add_system(system_b);
    }
}
