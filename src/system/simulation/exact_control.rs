use crate::{
    state::BasherState,
    system::{System, SystemSettings},
};

/// System to set the current UAV pose to it's desired pose
/// This acts a short-circuit to the control system and is used for testing
pub struct ExactControlSystem {}

impl ExactControlSystem {
    pub fn new() -> ExactControlSystem {
        ExactControlSystem {}
    }
}

impl System for ExactControlSystem {
    fn get_settings(&self) -> SystemSettings {
        SystemSettings::new(String::from("Exact Control"))
    }

    fn init(&mut self, state: &mut BasherState) {}

    fn execute(&mut self, state: &mut BasherState) {
        state.quad.quad_current_pose = state.quad.quad_desired_pose.clone();
    }

    fn cleanup(&mut self, state: &mut BasherState) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BasherState;
    use crate::system::mock::MockSystem;

    #[test]
    fn test_exact_control_system() {
        let mut state = BasherState::new();
        let mut system = ExactControlSystem::new();
        system.init(&mut state);

        // Set Desired Pose
        state.quad.quad_desired_pose.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        system.execute(&mut state);

        assert_eq!(
            state.quad.quad_current_pose.position,
            nalgebra::Vector3::new(0.0, 0.0, 10.0)
        );
        system.cleanup(&mut state);
    }
}
