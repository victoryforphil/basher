pub mod actor;
pub mod context;

use actor::{PhysicsActor, PhysicsActorSettings};
use context::RapierContext;
use log::info;
use nalgebra::Vector3;

use crate::{
    system::{System, SystemSettings},
    types::QuadPose,
};

pub struct PhysicSimulationSystem {
    pub rapier_context: RapierContext,
    pub quad: PhysicsActor,
    pub floor: PhysicsActor,
}

impl PhysicSimulationSystem {
    pub fn uav_actor_settings() -> PhysicsActorSettings {
        let settings = PhysicsActorSettings::new()
            .with_density(0.25)
            .with_size(nalgebra::Vector3::new(0.5, 0.5, 0.5));
        settings
    }

    pub fn floor_actor_settings() -> PhysicsActorSettings {
        let settings = PhysicsActorSettings::new()
            .with_size(nalgebra::Vector3::new(100.0, 100.0, 0.1))
            .with_initial_pose(
                QuadPose::new().with_position(nalgebra::Vector3::new(0.0, 0.0, -0.05)),
            );
        settings
    }

    pub fn new() -> Self {
        Self {
            rapier_context: RapierContext::new(),
            quad: PhysicsActor::new("UAV").with_settings(Self::uav_actor_settings()),
            floor: PhysicsActor::new("Floor"),
        }
    }
}

impl PhysicSimulationSystem {
    pub fn update_uav_actor(
        &mut self,
        state: &mut crate::state::BasherState,
        dt: victory_time_rs::Timespan,
    ) {
        // 1. Get current rigid body pose from actor:
        let uav_rigid = self
            .rapier_context
            .rigid_bodies
            .get_mut(self.quad.rigid_body)
            .unwrap();
        let gt_position = uav_rigid.position().translation.vector;
        let gt_orientation = uav_rigid.position().rotation;
        let gt_velocity = uav_rigid.linvel().clone_owned();
        let gt_angular_velocity = uav_rigid.angvel().clone_owned();

        let control_pose = state.quad.quad_desired_pose.clone();

        let desired_velocity = control_pose.velocity;
        // Apply acceleration to the rigid body
        let new_velocity = Vector3::new(
            desired_velocity.x as f32,
            desired_velocity.y as f32,
            desired_velocity.z as f32,
        );
        uav_rigid.set_linvel(new_velocity, true);

        // Update Quad state with GT
        let gt_pose = QuadPose::new()
            .with_position_f32(gt_position)
            .with_velocity_f32(Vector3::new(
                gt_velocity.x as f32,
                gt_velocity.y as f32,
                gt_velocity.z as f32,
            ))
            .with_orientation_f32(gt_orientation)
            .with_angular_velocity_f32(Vector3::new(
                gt_angular_velocity.x as f32,
                gt_angular_velocity.y as f32,
                gt_angular_velocity.z as f32,
            ));
        state.quad.quad_current_pose = gt_pose;
    }
}

impl System for PhysicSimulationSystem {
    fn init(&mut self, _state: &mut crate::state::BasherState) {
        info!("Initializing Physics Simulation System");
        self.quad.create(&mut self.rapier_context);
    }

    fn execute(&mut self, _state: &mut crate::state::BasherState, _dt: victory_time_rs::Timespan) {
        self.rapier_context.physics_pipeline.step(
            &self.rapier_context.gravity,
            &self.rapier_context.integration_params,
            &mut self.rapier_context.island_manager,
            &mut self.rapier_context.broad_phase,
            &mut self.rapier_context.narrow_phase,
            &mut self.rapier_context.rigid_bodies,
            &mut self.rapier_context.colliders,
            &mut self.rapier_context.impulse_joint_set,
            &mut self.rapier_context.multibody_join_set,
            &mut self.rapier_context.ccd_solve,
            Some(&mut self.rapier_context.query_pipeline),
            &self.rapier_context.physics_hooks,
            &self.rapier_context.ev,
        );

        self.update_uav_actor(_state, _dt);
    }

    fn cleanup(&mut self, _state: &mut crate::state::BasherState) {}

    fn get_settings(&self) -> SystemSettings {
        SystemSettings::new("Physics Simulation System".to_string())
    }
}
