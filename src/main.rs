mod quad;
mod state;
mod system;
use ::log::info;
use ::log::LevelFilter;
use ::rerun::{Scalar, SeriesPoint};
use basher_rerun::BasherRerun;
use rerun::*;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
mod basher_rerun;
fn main() {
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

    info!("[Basher Main] Starting Basher...");

    for i in 0..10 {
        info!("[Basher Main] Iteration {}", i);
        rerun.rerun().unwrap().set_time_seconds("time", i);
        rerun
            .rerun()
            .unwrap()
            .log("test_data", &Scalar::new(i as f64))
            .expect("Failed to log data");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
