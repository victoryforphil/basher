use log::info;

fn main() {
    env_logger::init();
    info!("[Basher Main] Starting Basher...")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
