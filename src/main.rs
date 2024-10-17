mod state;
mod system;
mod types;

use ::log::info;
use nalgebra::Vector3;

use state::commander::Mission;

use state::BasherState;
use system::quad::CommanderSystem;
use system::quad::MockQuad;
use system::simulation::physics;
use system::viz::rerun_system::RerunSystem;
use victory_commander::system::runner::BasherSysRunner;
use victory_data_store::topics::TopicKey;
use victory_time_rs::Timepoint;
use victory_time_rs::Timespan;
mod basher_rerun;
fn main() {
    pretty_env_logger::init();

    info!("[Basher Main] Starting Basher...");

    let mission = Mission::new_simple(vec![
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(5.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(-10.0, -10.0, 10.0),
        Vector3::new(10.0, 10.0, 10.0),
        Vector3::new(2.0, -2.0, 2.0),
    ]);

    let commander_system = CommanderSystem::new().with_mission(mission);
    let quad_system = MockQuad::new();
    // let physics_system = ExactControlSystem::new_acceleration();
    let physics_system = physics::PhysicSimulationSystem::new();

    let rerun_system = RerunSystem::new("basher".to_string());
    let mut runner = BasherSysRunner::new();
    runner.add_system(std::sync::Arc::new(std::sync::Mutex::new(commander_system)));
    runner.add_system(std::sync::Arc::new(std::sync::Mutex::new(quad_system)));
    runner.add_system(std::sync::Arc::new(std::sync::Mutex::new(physics_system)));
    runner.add_system(std::sync::Arc::new(std::sync::Mutex::new(rerun_system)));
    // Set desired post to be 5m above current quad position
    runner.dt = Timespan::new_hz(1.0);
    runner.run(Timepoint::new_secs(10.0));
    // Dump datastore keys
    let keys = runner.data_store.get_all_keys();
    info!("[Basher Main] Datastore keys: {:#?}", keys);
    let state: BasherState = runner.data_store.get_struct(&TopicKey::from_str("basher_state")).unwrap();
    info!("[Basher Main] Quad final position: {:?}", state.quad.quad_current_pose.position);
    info!("[Basher Main] Basher finished");
    // Print final quad position
    info!(
        "[Basher Main] Quad final position: {:?}",
        state.quad.quad_current_pose.position
    );

    // Print current time
    info!(
        "[Basher Main] Current time: {:?}",
        runner.current_time
    );
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        // main();
    }
}
