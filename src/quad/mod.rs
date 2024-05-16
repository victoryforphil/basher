use pose::QuadPose;

use crate::basher_rerun::BashRerunHandle;

pub mod pose;
pub mod mock;
pub enum PhysicsOutput{
    DesiredPose(QuadPose)
}


pub struct QuadInputs{
    pub rerun: BashRerunHandle,
    pub pose: QuadPose
}


pub struct QuadOutputs{
    pub physics: PhysicsOutput
}
pub trait Quad{
    fn on_update(&mut self, inputs: QuadInputs) -> QuadOutputs;
}