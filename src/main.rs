mod state;
mod system;
mod types;

use ::log::info;
use ::log::LevelFilter;
use ::rerun::{Scalar, SeriesPoint};
use nalgebra::Vector3;

use rerun::*;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use state::commander::Mission;

use system::quad::CommanderSystem;
use system::quad::MockQuad;
use system::runner::BasherSysRunner;
use system::simulation::exact_control::ExactControlSystem;
use system::viz::rerun_system::RerunSystem;
use victory_time_rs::Timespan;
mod basher_rerun;
fn main() {
    // pretty_env_logger::init();

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
    let physics_system = ExactControlSystem::new_acceleration();
    let rerun_system = RerunSystem::new("basher".to_string());
    let mut runner = BasherSysRunner::new();
    runner.add_system(Box::new(commander_system));
    runner.add_system(Box::new(quad_system));
    runner.add_system(Box::new(physics_system));
    runner.add_system(Box::new(rerun_system));
    // Set desired post to be 5m above current quad position
    runner.run(Timespan::new_secs(30.0));

    info!("[Basher Main] Basher finished");
    // Print final quad position
    info!(
        "[Basher Main] Quad final position: {:?}",
        runner.state.quad.quad_current_pose.position
    );

    // Print current time
    info!(
        "[Basher Main] Current time: {:?}",
        runner.state.current_time
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
       // main();
    }
}
