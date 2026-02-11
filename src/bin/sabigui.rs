#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sabikui::gui::constants::{CONFIG_DIRECTORY, LOG_FILE};
use simplelog::Config;

fn main() {
    let log_path = dirs::config_dir().unwrap().join(CONFIG_DIRECTORY).join(LOG_FILE);
    simplelog::WriteLogger::init(
        log::LevelFilter::Error,
        Config::default(),
        std::fs::File::create(&log_path).unwrap(),
    )
    .unwrap();

    sabikui::gui::run();
}
