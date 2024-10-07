use log::warn;
use victory_time_rs::Timespan;

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

    fn execute(&mut self, state: &mut crate::state::BasherState, _dt: Timespan) {
        let rerun = &mut self.basher_rerun.rerun;

        let rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return;
        };

        rerun.set_time_seconds("system-time", state.current_time.secs());
        
        {
            let current_viz:RerunQuadPose = state.quad.quad_current_pose.clone().into();
            current_viz.log_pose("current", rerun);
            
            let desired_viz:RerunQuadPose = state.quad.quad_desired_pose.clone().into();
            desired_viz.log_pose("desired", rerun);

            let goal_viz:RerunQuadPose = state.quad.quad_goal_pose.clone().into();
            goal_viz.log_pose("goal", rerun);
        }
    }

    fn cleanup(&mut self, state: &mut crate::state::BasherState) {}
}
