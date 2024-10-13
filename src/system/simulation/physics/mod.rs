pub mod context;
pub mod actor;

use actor::{PhysicsActor, PhysicsActorSettings};

use crate::system::{System, SystemSettings};

use super::physics_sim::RapierContext;

pub struct PhysicSimulationSystem{
    pub rapier_context: RapierContext,
    pub actors: Vec<PhysicsActor>,
}

impl PhysicSimulationSystem {
    pub fn new() -> Self {
        Self {
            rapier_context: RapierContext::new(),
            actors: Vec::new(),
        }
    }

    pub fn add_actro(&mut self, name: impl Into<String>, settings: PhysicsActorSettings) -> &mut Self {
        let mut actor = PhysicsActor::new(name.into());
        actor.settings = settings;
        self.actors.push(actor);
        self
    }
}

impl System for PhysicSimulationSystem{
    fn init(&mut self, state: &mut crate::state::BasherState) {
        
    }

    fn execute(&mut self, state: &mut crate::state::BasherState, dt: victory_time_rs::Timespan) {
        
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
        )
    }

    fn cleanup(&mut self, state: &mut crate::state::BasherState) {
        
    }
    
    fn get_settings(&self) -> SystemSettings {
        SystemSettings::new("Physics Simulation System".to_string())
    }
}