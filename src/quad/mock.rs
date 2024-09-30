use super::Quad;

pub struct MockQuad {
    pub goal: nalgebra::Vector3<f64>,
    pub speed: f64,
}

impl MockQuad {
    pub fn new(goal: nalgebra::Vector3<f64>, speed: f64) -> MockQuad {
        MockQuad { goal, speed }
    }
}

impl Quad for MockQuad {
    fn on_update(&mut self, inputs: super::QuadInputs) -> super::QuadOutputs {
        let position = inputs.pose.position;
        let direction = self.goal - position;
        let velocity = direction.normalize() * self.speed;
        let acceleration = velocity - inputs.pose.velocity;
        let output = super::PhysicsOutput::DesiredPose(super::pose::QuadPose {
            position,
            orientation: inputs.pose.orientation,
            velocity,
            angular_velocity: inputs.pose.angular_velocity,
            acceleration,
            angular_acceleration: inputs.pose.angular_acceleration,
        });
        super::QuadOutputs { physics: output }
    }
}
