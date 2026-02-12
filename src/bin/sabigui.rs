#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sabikui::gui::constants::{CONFIG_DIRECTORY, LOG_FILE};

fn main() {
    let log_path = dirs::config_dir()
        .expect("There should be a config directory")
        .join(CONFIG_DIRECTORY)
        .join(LOG_FILE);
    simplelog::WriteLogger::init(
        log::LevelFilter::Error,
        simplelog::Config::default(),
        std::fs::File::create(&log_path).expect("We should be able to open the log file"),
    )
    .expect("The logger initialization should not fail");
    log_panics::init();

    sabikui::gui::run();
}
