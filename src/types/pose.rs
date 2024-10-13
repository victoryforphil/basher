use rerun::{AsComponents, Points3D, Vec3D};

#[derive(Debug, Clone)]
pub struct QuadPose {
    pub position: nalgebra::Vector3<f64>,
    pub orientation: nalgebra::UnitQuaternion<f64>,
    pub velocity: nalgebra::Vector3<f64>,
    pub angular_velocity: nalgebra::Vector3<f64>,
    pub acceleration: nalgebra::Vector3<f64>,
    pub angular_acceleration: nalgebra::Vector3<f64>,
}

impl Default for QuadPose {
    fn default() -> Self {
        QuadPose {
            position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            orientation: nalgebra::UnitQuaternion::identity(),
            velocity: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            acceleration: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            angular_acceleration: nalgebra::Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl QuadPose {
    /// Create a new QuadPose with default values
    pub fn new() -> QuadPose {
        QuadPose::default()
    }

    /// Create a new QuadPose with the given values
    pub fn with_position(mut self, position: nalgebra::Vector3<f64>) -> QuadPose {
        self.position = position;
        self
    }

    pub fn with_orientation(mut self, orientation: nalgebra::UnitQuaternion<f64>) -> QuadPose {
        self.orientation = orientation;
        self
    }

    pub fn with_velocity(mut self, velocity: nalgebra::Vector3<f64>) -> QuadPose {
        self.velocity = velocity;
        self
    }

    pub fn with_angular_velocity(mut self, angular_velocity: nalgebra::Vector3<f64>) -> QuadPose {
        self.angular_velocity = angular_velocity;
        self
    }

    pub fn with_acceleration(mut self, acceleration: nalgebra::Vector3<f64>) -> QuadPose {
        self.acceleration = acceleration;
        self
    }

    pub fn with_angular_acceleration(
        mut self,
        angular_acceleration: nalgebra::Vector3<f64>,
    ) -> QuadPose {
        self.angular_acceleration = angular_acceleration;
        self
    }

    pub fn position_f32(&self) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(
            self.position.x as f32,
            self.position.y as f32,
            self.position.z as f32,
        )
    }

    pub fn angel_axis_f32(&self) -> nalgebra::Vector3<f32> {
        let (x,y,z) =  self.orientation.euler_angles();
        nalgebra::Vector3::new(x as f32, y as f32, z as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_pose() {
        let pose = QuadPose::new()
            .with_position(nalgebra::Vector3::new(1.0, 2.0, 3.0))
            .with_orientation(nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0))
            .with_velocity(nalgebra::Vector3::new(4.0, 5.0, 6.0))
            .with_angular_velocity(nalgebra::Vector3::new(7.0, 8.0, 9.0))
            .with_acceleration(nalgebra::Vector3::new(10.0, 11.0, 12.0))
            .with_angular_acceleration(nalgebra::Vector3::new(13.0, 14.0, 15.0));
        assert_eq!(pose.position, nalgebra::Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(
            pose.orientation,
            nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0)
        );
        assert_eq!(pose.velocity, nalgebra::Vector3::new(4.0, 5.0, 6.0));
        assert_eq!(pose.angular_velocity, nalgebra::Vector3::new(7.0, 8.0, 9.0));
        assert_eq!(pose.acceleration, nalgebra::Vector3::new(10.0, 11.0, 12.0));
        assert_eq!(
            pose.angular_acceleration,
            nalgebra::Vector3::new(13.0, 14.0, 15.0)
        );
    }
}
