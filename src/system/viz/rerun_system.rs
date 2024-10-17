use std::collections::BTreeSet;

use log::warn;
use serde::{Deserialize, Serialize};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_time_rs::{Timepoint, Timespan};

use crate::{basher_rerun::{types::RerunQuadPose, BasherRerun}, state::BasherState};



pub struct RerunSystem {
    pub basher_rerun: BasherRerun,
}

impl RerunSystem {
    pub fn new(name: String) -> RerunSystem {
        let run_id = Timepoint::now().ms().to_string();
        RerunSystem {
            basher_rerun: BasherRerun::new(name, "default-group".to_string(), run_id),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmptyOut{

}




impl System for RerunSystem {
 
    fn init(&mut self) {
        self.basher_rerun.create_rerun();
    }

    fn execute(&mut self, state: &DataView, _dt: Timespan) -> DataView {
        
        let mut state: BasherState = state.get_latest(&TopicKey::from_str("basher_state")).unwrap();
        let rerun = &mut self.basher_rerun.rerun;

        let rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return DataView::new();
        };

        rerun.set_time_seconds("system-time", state.current_time.secs());

        {
            let current_viz: RerunQuadPose = state.quad.quad_current_pose.clone().into();
            current_viz.log_pose("current", rerun);

            let desired_viz: RerunQuadPose = state.quad.quad_desired_pose.clone().into();
            desired_viz.log_pose("desired", rerun);

            let goal_viz: RerunQuadPose = state.quad.quad_goal_pose.clone().into();
            goal_viz.log_pose("goal", rerun);
        }
        {
            for (i, waypoint) in state.commander.mission.waypoints.iter().enumerate() {
                let waypoint_viz: RerunQuadPose = waypoint.pose.clone().into();
                waypoint_viz.log_pose(&format!("waypoint-{}", i), rerun);
            }
        }
        DataView::new()
    }

    fn cleanup(&mut self) {}
    
    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("basher_state"));
        topics
    }
}
