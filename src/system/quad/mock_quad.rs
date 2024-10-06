use log::info;

use crate::{
    system::{System, SystemSettings},
    types::QuadPose,
};

pub struct MockQuad {}

impl MockQuad {
    pub fn new() -> MockQuad {
        MockQuad {}
    }
}

impl System for MockQuad {
    fn get_settings(&self) -> crate::system::SystemSettings {
        SystemSettings::new(String::from("Mock Quad"))
    }

    fn init(&mut self, state: &mut crate::state::BasherState) {
        info!("Initializing Mock Quad");
    }

    fn execute(&mut self, state: &mut crate::state::BasherState) {
        let position = state.quad.quad_current_pose.position;
        let direction = state.quad.quad_goal_pose.position - position;
        let velocity = direction.normalize() * state.quad.setting_max_lat_vel;
        let acceleration = velocity - state.quad.quad_current_pose.velocity;

        state.quad.quad_desired_pose = QuadPose::new()
            .with_position(state.quad.quad_goal_pose.position)
            .with_velocity(velocity)
            .with_acceleration(acceleration)
            .with_orientation(state.quad.quad_current_pose.orientation)
            .with_angular_velocity(state.quad.quad_current_pose.angular_velocity)
            .with_angular_acceleration(state.quad.quad_current_pose.angular_acceleration);
    }

    fn cleanup(&mut self, state: &mut crate::state::BasherState) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BasherState;

    #[test]
    fn test_mock_quad_smoketest() {
        let mut state = BasherState::new();
        let mut system = MockQuad::new();
        system.init(&mut state);
        system.execute(&mut state);
        system.cleanup(&mut state);
    }

    #[test]
    fn test_mock_quad_vert_test() {
        let mut state = BasherState::new();
        let mut system = MockQuad::new();
        system.init(&mut state);
        state.quad.quad_current_pose.position = nalgebra::Vector3::new(0.0, 0.0, 0.0);
        state.quad.quad_goal_pose.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        state.quad.setting_max_vert_vel = 1.0;
        system.execute(&mut state);

        system.cleanup(&mut state);
    }
}
