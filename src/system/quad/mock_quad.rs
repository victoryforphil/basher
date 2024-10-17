use std::collections::BTreeSet;

use log::info;
use nalgebra::Vector3;
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_time_rs::Timespan;

use crate::{
    state::BasherState,types::QuadPose
};

pub struct MockQuad {}

impl MockQuad {
    pub fn new() -> MockQuad {
        MockQuad {}
    }
}

impl System for MockQuad {

    fn name(&self) -> String {
        "MockQuad".to_string()
    }

    fn init(&mut self) {
        info!("Initializing Mock Quad");
    }
    fn execute(&mut self, state: &DataView, _dt: Timespan) -> DataView {
        let mut state: BasherState = state.get_latest(&TopicKey::from_str("basher_state")).unwrap();
        let position = state.quad.quad_current_pose.position;
        let mut direction = state.quad.quad_goal_pose.position - position;

        if direction.norm() < 0.08 {
            state.quad.quad_desired_pose = QuadPose::new()
                .with_position(state.quad.quad_goal_pose.position)
                .with_velocity(Vector3::new(0.0, 0.0, 0.0));
            let mut dataview = DataView::new();
            dataview.add_latest(&TopicKey::from_str("basher_state"), &state);
            return dataview;
        }
        // Clamp the direction vector to the max velocity
        direction = direction.normalize() * state.quad.setting_max_lat_vel;

        let velocity = direction;
        let acceleration = velocity - state.quad.quad_current_pose.velocity;
      
        state.quad.quad_desired_pose = QuadPose::new()
            .with_position(state.quad.quad_goal_pose.position)
            .with_velocity(velocity)
            .with_acceleration(acceleration)
            .with_orientation(state.quad.quad_current_pose.orientation)
            .with_angular_velocity(state.quad.quad_current_pose.angular_velocity)
            .with_angular_acceleration(state.quad.quad_current_pose.angular_acceleration);
        let mut dataview = DataView::new();
        dataview.add_latest(&TopicKey::from_str("basher_state"), &state);
        dataview
    }
    

    fn cleanup(&mut self) {}

    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("basher_state"));
        topics
    }
  
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BasherState;

    #[test]
    fn test_mock_quad_smoketest() {
        let mut state = BasherState::new();
        let mut system = MockQuad::new();
        system.init();
        let mut dataview = DataView::new();
        dataview.add_latest(&TopicKey::from_str("basher_state"), &state);
        system.execute(&dataview, Timespan::zero());
        system.cleanup();
    }

    #[test]
    fn test_mock_quad_vert_test() {
        let mut state = BasherState::new();
        let mut system = MockQuad::new();
        system.init();
        state.quad.quad_current_pose.position = nalgebra::Vector3::new(0.0, 0.0, 0.0);
        state.quad.quad_goal_pose.position = nalgebra::Vector3::new(0.0, 0.0, 10.0);
        state.quad.setting_max_vert_vel = 1.0;
        let mut dataview = DataView::new();
        dataview.add_latest(&TopicKey::from_str("basher_state"), &state);
        system.execute(&dataview, Timespan::new_hz(10.));

        system.cleanup();
    }
}
