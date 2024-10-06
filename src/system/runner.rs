use std::time::{Duration, Instant};

use log::info;

use crate::state::BasherState;

use super::SystemHandle;

pub struct BasherSysRunner {
    pub systems: Vec<SystemHandle>,
    pub state: BasherState,
    end_time: Instant,
}

impl BasherSysRunner {
    pub fn new() -> BasherSysRunner {
        BasherSysRunner {
            systems: Vec::new(),
            state: BasherState::new(),
            end_time: Instant::now(),
        }
    }

    pub fn add_system(&mut self, system: SystemHandle) {
        info!("Adding System: {}", system.get_settings().name);
        self.systems.push(system);
    }

    pub fn run(&mut self, duration: Duration) {
        self.end_time = Instant::now() + duration;
        for system in self.systems.iter_mut() {
            system.init(&mut self.state);
        }

        while Instant::now() < self.end_time {
            for system in self.systems.iter_mut() {
                system.execute(&mut self.state);
            }
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
        runner.run(Duration::from_secs(1));
    }
}
