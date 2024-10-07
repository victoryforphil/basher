use log::warn;

use crate::{
    basher_rerun::{types::RerunQuadPose, BasherRerun},
    system::{System, SystemSettings},
};

pub struct RerunSystem {
    pub basher_rerun: BasherRerun,
}

impl RerunSystem {
    pub fn new(name: String) -> RerunSystem {
        RerunSystem {
            basher_rerun: BasherRerun::new(name, "default-group".to_string(), "0".to_string()),
        }
    }
}

impl System for RerunSystem {
    fn get_settings(&self) -> crate::system::SystemSettings {
        SystemSettings::new(String::from("Rerun System"))
    }

    fn init(&mut self, state: &mut crate::state::BasherState) {
        self.basher_rerun.create_rerun();
    }

    fn execute(&mut self, state: &mut crate::state::BasherState) {
        let rerun = &mut self.basher_rerun.rerun;

        let rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return;
        };

        rerun.set_time_seconds("system-time", state.current_time.secs());
        rerun.log(
            "quad/pose/current",
            &RerunQuadPose::from(state.quad.quad_current_pose.clone()),
        );
        rerun.log(
            "quad/pose/goal",
            &RerunQuadPose::from(state.quad.quad_goal_pose.clone()),
        );
        rerun.log(
            "quad/pose/desired",
            &RerunQuadPose::from(state.quad.quad_desired_pose.clone()),
        );
    }

    fn cleanup(&mut self, state: &mut crate::state::BasherState) {}
}
