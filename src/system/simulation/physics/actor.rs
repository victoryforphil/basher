use log::info;
use nalgebra::Vector3;
use rapier3d::prelude::{ColliderBuilder, ColliderHandle, LockedAxes, RigidBodyBuilder, RigidBodyHandle};

use crate::types::QuadPose;

use super::context::RapierContext;

#[derive(Clone, Debug)]
pub struct PhysicsActorSettings {
    pub initial_pose: QuadPose,
    pub density: f64,
    pub size: Vector3<f64>,
    pub locked_rotations: Vector3<bool>,
    pub locked_translations: Vector3<bool>,
}

impl PhysicsActorSettings {
    pub fn new() -> Self {
        Self {
            initial_pose: QuadPose::new(),
            density: 1.0,
            size: Vector3::new(1.0, 1.0, 1.0),
            locked_rotations: Vector3::new(false, false, false),
            locked_translations: Vector3::new(false, false, false),
        }
    }

    pub fn with_initial_pose(mut self, initial_pose: QuadPose) -> Self {
        self.initial_pose = initial_pose;
        self
    }

    pub fn with_density(mut self, density: f64) -> Self {
        self.density = density;
        self
    }

    pub fn with_size(mut self, size: Vector3<f64>) -> Self {
        self.size = size;
        self
    }

    pub fn with_locked_rotations(mut self, locked_rotations: Vector3<bool>) -> Self {
        self.locked_rotations = locked_rotations;
        self
    }

    pub fn with_locked_translations(mut self, locked_translations: Vector3<bool>) -> Self {
        self.locked_translations = locked_translations;
        self
    }

    pub fn build_locked_axis(&self) -> LockedAxes{
        let mut axis =  LockedAxes::empty();
        axis.set(LockedAxes::TRANSLATION_LOCKED_X, self.locked_translations.x);
        axis.set(LockedAxes::TRANSLATION_LOCKED_Y, self.locked_translations.y);
        axis.set(LockedAxes::TRANSLATION_LOCKED_Z, self.locked_translations.z);
        axis.set(LockedAxes::ROTATION_LOCKED_X, self.locked_rotations.x);
        axis.set(LockedAxes::ROTATION_LOCKED_Y, self.locked_rotations.y);
        axis.set(LockedAxes::ROTATION_LOCKED_Z, self.locked_rotations.z);
        axis
    }

    pub fn build_rigidbody(&self) -> RigidBodyBuilder {
        RigidBodyBuilder::dynamic()
        .translation(self.initial_pose.position_f32())
        .rotation(self.initial_pose.angel_axis_f32())
        .locked_axes(self.build_locked_axis())
    }

    pub fn build_collider(&self) -> ColliderBuilder{
        let (s_x, s_y, s_z) = (self.size.x as f32, self.size.y as f32, self.size.z as f32);
        ColliderBuilder::cuboid(s_x, s_y, s_z).density(self.density as f32)
    }
}

pub struct PhysicsActor {
    pub name: String,
    pub rigid_body: RigidBodyHandle,
    pub settings: PhysicsActorSettings,
    pub colliders: Vec<ColliderHandle>,
}

impl PhysicsActor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            settings: PhysicsActorSettings::new(),
            rigid_body: RigidBodyHandle::from_raw_parts(0, 0),
            colliders: Vec::new(),
        }
    }

    pub fn settings(&mut self) -> &mut PhysicsActorSettings {
        &mut self.settings
    }
    pub fn create(&mut self, context: &mut RapierContext) {
        info!("Creating actor: {}", self.name);
        let settings = &self.settings;
        let rigid_body = settings.build_rigidbody().build();
        let collider = settings.build_collider().build();
        
        let rigid_body_handle = context.rigid_bodies.insert(rigid_body);
        let collider_handle = context.colliders.insert_with_parent(collider, rigid_body_handle, &mut context.rigid_bodies);
        self.rigid_body = rigid_body_handle;
        self.colliders.push(collider_handle);
    }
}
