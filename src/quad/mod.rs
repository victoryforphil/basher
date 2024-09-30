use pose::QuadPose;

use crate::basher_rerun::BashRerunHandle;

pub mod mock;
pub mod pose;
pub enum PhysicsOutput {
    DesiredPose(QuadPose),
}

pub struct QuadInputs {
    pub rerun: BashRerunHandle,
    pub pose: QuadPose,
}

pub struct QuadOutputs {
    pub physics: PhysicsOutput,
}
pub trait Quad {
    fn on_update(&mut self, inputs: QuadInputs) -> QuadOutputs;
}
