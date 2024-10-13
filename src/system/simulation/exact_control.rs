use victory_time_rs::Timespan;

use crate::{
    state::BasherState,
    system::{System, SystemSettings},
};

pub enum ExactControlMode {
    DesiredPosition,
    DesiredVelocity,
    DesiredAcceleration,
}

/// System to set the current UAV pose to it's desired pose
/// This acts a short-circuit to the control system and is used for testing
pub struct ExactControlSystem {
    pub control_mode: ExactControlMode,
}

impl ExactControlSystem {
    pub fn new() -> ExactControlSystem {
        ExactControlSystem {
            control_mode: ExactControlMode::DesiredPosition,
        }
    }

    pub fn new_velocity() -> ExactControlSystem {
        ExactControlSystem {
            control_mode: ExactControlMode::DesiredVelocity,
        }
    }

    pub fn new_acceleration() -> ExactControlSystem {
        ExactControlSystem {
            control_mode: ExactControlMode::DesiredAcceleration,
        }
    }
}

impl System for ExactControlSystem {
    fn get_settings(&self) -> SystemSettings {
        SystemSettings::new(String::from("Exact Control"))
    }

    fn init(&mut self, state: &mut BasherState) {}

    fn execute(&mut self, state: &mut BasherState, dt: Timespan) {
        match self.control_mode {
            ExactControlMode::DesiredPosition => {
                state.quad.quad_current_pose.position =
                    state.quad.quad_desired_pose.position.clone();
            }
            ExactControlMode::DesiredVelocity => {
                state.quad.quad_current_pose.velocity =
                    state.quad.quad_desired_pose.velocity.clone();
                //Based on the desired velocity, update the position according to dt
                state.quad.quad_current_pose.position +=
                    state.quad.quad_current_pose.velocity * dt.secs();
            }
            ExactControlMode::DesiredAcceleration => {
                state.quad.quad_current_pose.acceleration =
                    state.quad.quad_desired_pose.acceleration.clone();
                //Based on the desired acceleration, update the velocity according to dt
                state.quad.quad_current_pose.velocity +=
                    state.quad.quad_current_pose.acceleration * dt.secs();
                //Based on the desired velocity, update the position according to dt
                state.quad.quad_current_pose.position +=
                    state.quad.quad_current_pose.velocity * dt.secs();
            }
        }
    }

    fn cleanup(&mut self, state: &mut BasherState) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BasherState;
    

    #[test]
    fn test_exact_control_system() {
        let mut state = BasherState::new();
        let mut system = ExactControlSystem::new();
        system.init(&mut state);

        // Set Desired Pose
        state.quad.quad_desired_pose.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        system.execute(&mut state, Timespan::new_hz(100.0));

        assert_eq!(
            state.quad.quad_current_pose.position,
            nalgebra::Vector3::new(0.0, 0.0, 10.0)
        );
        system.cleanup(&mut state);
    }
}
