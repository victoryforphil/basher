use nalgebra::Vector3;
use rerun::{AsComponents, Points3D, RecordingStream, Scalar, SeriesPoint, Vec3D};

use crate::types::QuadPose;

pub struct RerunQuadPose {
 
    pub pose: QuadPose,
}

impl RerunQuadPose {
    pub fn new(pose: QuadPose) -> RerunQuadPose {
        let pos = pose.position;
    
        RerunQuadPose {

            pose: pose,

        }
    }
    
    pub fn log_scalar(&self, prefix: &str, stream: &mut RecordingStream, name: &str, value: f64) {
        let scalar = Scalar::new(value);
        let scalar_name = format!("{}/{}", prefix, name);
        stream.log(scalar_name, &scalar);
    }

    pub fn log_vector(&self, prefix: &str, stream: &mut RecordingStream, name: &str,vec: Vector3<f64>) {
        let point = Points3D::new([Vec3D::new(vec.x as f32, vec.y as f32, vec.z as f32)]).with_labels(vec!["position"]);
        let point_name = format!("{}/{}", prefix, name);
        stream.log(point_name, &point);
        let prefix = format!("{}/{}", prefix, name);
        self.log_scalar(prefix.as_str(), stream, "x", vec.x as f64);
        self.log_scalar(prefix.as_str(), stream, "y", vec.y as f64);
        self.log_scalar(prefix.as_str(), stream, "z", vec.z as f64);
    }

    pub fn log_pose(&self,prefix: &str, stream: &mut RecordingStream) {
        self.log_vector(prefix, stream, "position", self.pose.position);
        self.log_vector(prefix, stream, "velocity", self.pose.velocity);
        self.log_vector(prefix, stream, "angular_velocity", self.pose.angular_velocity);
        self.log_vector(prefix, stream, "acceleration", self.pose.acceleration);
        self.log_vector(prefix, stream, "angular_acceleration", self.pose.angular_acceleration);
    }
}

impl From<QuadPose> for RerunQuadPose {
    fn from(pose: QuadPose) -> Self {
        RerunQuadPose::new(pose)
    }
}


