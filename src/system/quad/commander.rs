use crate::{
    state::{commander::Mission, BasherState},
    system::{System, SystemSettings},
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
    fn get_settings(&self) -> crate::system::SystemSettings {
        SystemSettings::new("Commander".to_string())
    }

    fn init(&mut self, state: &mut BasherState) {
        if let Some(mission) = &self.mission {
            state.commander.mission = mission.clone();
        }
    }

    fn execute(&mut self, state: &mut BasherState, dt: victory_time_rs::Timespan) {
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
    }

    fn cleanup(&mut self, state: &mut BasherState) {
    }
}
