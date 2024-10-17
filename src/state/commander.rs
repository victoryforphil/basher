use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::types::QuadPose;

use super::waypoint::Waypoint;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mission {
    pub waypoints: Vec<Waypoint>,
}

impl Mission {
    pub fn new() -> Mission {
        Mission {
            waypoints: Vec::new(),
        }
    }
    pub fn with_waypoints(mut self, waypoints: Vec<Waypoint>) -> Mission {
        self.waypoints = waypoints;
        self
    }

    pub fn with_waypoint(mut self, waypoint: Waypoint) -> Mission {
        self.waypoints.push(waypoint);
        self
    }

    pub fn new_simple(positions: Vec<Vector3<f64>>) -> Mission {
        let mut waypoints = Vec::new();
        for position in positions {
            let pose = QuadPose::new().with_position(position);
            waypoints.push(Waypoint::new(pose, 0.1));
        }

        Mission::new().with_waypoints(waypoints)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommanderState {
    pub mission: Mission,
    pub current_target_index: usize,
}
impl Default for CommanderState {
    fn default() -> Self {
        CommanderState {
            mission: Mission::new(),
            current_target_index: 0,
        }
    }
}
impl CommanderState {
    pub fn new() -> CommanderState {
        CommanderState {
            mission: Mission::new(),
            current_target_index: 0,
        }
    }
    pub fn with_mission(mut self, mission: Mission) -> CommanderState {
        self.mission = mission;
        self
    }
}
