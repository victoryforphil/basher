use std::collections::BTreeSet;

use log::{debug, info};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_time_rs::Timespan;

use crate::{
    state::{commander::Mission, BasherState},
};

pub struct CommanderSystem {
    pub mission: Option<Mission>,
}

impl CommanderSystem {
    pub fn new() -> CommanderSystem {
        CommanderSystem { mission: None }
    }
    pub fn with_mission(mut self, mission: Mission) -> CommanderSystem {
        self.mission = Some(mission);
        self
    }
}

impl System for CommanderSystem {

    fn name(&self) -> String {
        "Commander".to_string()
    }

    fn init(&mut self) {
        
    }

    fn execute(&mut self, state: &DataView, dt: Timespan) -> DataView {
        debug!("[Commander] Executing: state");
        let mut state: BasherState = state.get_latest(&TopicKey::from_str("basher_state")).unwrap_or_default();
        if let Some(mission) = &self.mission {
            state.commander.mission = mission.clone();
        }
        let mission = &state.commander.mission;
        let current_waypoint = &mission.waypoints[state.commander.current_target_index];
        let current_pose = &state.quad.quad_current_pose;
        let target_pose = &current_waypoint.pose;

        let distance = (current_pose.position - target_pose.position).norm();

        if distance < 0.1 {
            state.commander.current_target_index += 1;
            if state.commander.current_target_index >= mission.waypoints.len() {
                state.commander.current_target_index = mission.waypoints.len() - 1;
            }
        }

        // Set the goal pose to the current target
        state.quad.quad_goal_pose = target_pose.clone();
        let mut dataview = DataView::new();
        dataview.add_latest(&TopicKey::from_str("basher_state"), &state);
        dataview
    }

    fn cleanup(&mut self) {}
    
    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<victory_data_store::topics::TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("basher_state"));
        topics
    }
}
