mod state;
mod system;
mod types;

use ::log::info;
use ::log::LevelFilter;
use ::rerun::{Scalar, SeriesPoint};
use basher::system::quad::MockQuad;
use basher::system::runner::BasherSysRunner;
use basher_rerun::BasherRerun;
use rerun::*;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
mod basher_rerun;
fn main() {
    pretty_env_logger::init();
    /* TODO: Re-enable once rerun is a system
    //TOOD: Expose this to its own configurable logging setting, lua?
    let mode = BasherRerun::get_rerun_env();
    if mode == basher_rerun::RerunMode::Save {
        pretty_env_logger::init();
        info!("[Basher Main] Running in save mode, using CLI based rerun");
    }

    let mut rerun = basher_rerun::BasherRerun::new(
        "basher".to_string(),
        "manual".to_string(),
        "999".to_string(),
    );
    rerun.create_rerun();
    */
    info!("[Basher Main] Starting Basher...");

    let quad_system = MockQuad::new();
    let mut runner = BasherSysRunner::new();
    runner.add_system(Box::new(quad_system));

    // Set desired post to be 5m above current quad position
    runner.state.quad.quad_goal_pose.position = nalgebra::Vector3::new(0.0, 0.0, 5.0);

    runner.run(std::time::Duration::from_secs(5));

    info!("[Basher Main] Basher finished");
    // Print final quad position
    info!(
        "[Basher Main] Quad final position: {:?}",
        runner.state.quad.quad_current_pose.position
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
