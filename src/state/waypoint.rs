
use crate::types::QuadPose;

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub pose: QuadPose,
    pub accuracy: f64,
}

impl Waypoint {
    pub fn new(position: QuadPose, accuracy: f64) -> Waypoint {
        Waypoint {
            pose: position,
            accuracy: accuracy,
        }
    }
    pub fn with_pose(mut self, pose: QuadPose) -> Waypoint {
        self.pose = pose;
        self
    }
    pub fn with_accuracy(mut self, accuracy: f64) -> Waypoint {
        self.accuracy = accuracy;
        self
    }
}
