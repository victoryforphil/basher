use crate::types::QuadPose;
#[derive(Debug)]
pub struct QuadState {
    pub quad_current_pose: QuadPose,
    pub quad_goal_pose: QuadPose,
    pub quad_desired_pose: QuadPose,
    pub setting_max_lat_vel: f64,
    pub setting_max_vert_vel: f64,
}

impl QuadState {
    pub fn new() -> QuadState {
        QuadState {
            quad_current_pose: QuadPose::new(),
            quad_goal_pose: QuadPose::new(),
            quad_desired_pose: QuadPose::new(),
            setting_max_lat_vel: 5.0,
            setting_max_vert_vel: 2.0,
        }
    }
    pub fn with_quad_current_pose(mut self, quad_current_pose: QuadPose) -> QuadState {
        self.quad_current_pose = quad_current_pose;
        self
    }
    pub fn with_quad_goal_pose(mut self, quad_goal_pose: QuadPose) -> QuadState {
        self.quad_goal_pose = quad_goal_pose;
        self
    }
    pub fn with_setting_max_lat_vel(mut self, setting_max_lat_vel: f64) -> QuadState {
        self.setting_max_lat_vel = setting_max_lat_vel;
        self
    }

    pub fn with_setting_max_vert_vel(mut self, setting_max_vert_vel: f64) -> QuadState {
        self.setting_max_vert_vel = setting_max_vert_vel;
        self
    }
}
