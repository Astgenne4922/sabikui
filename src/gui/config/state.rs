use std::fs;

use crate::gui::{constants, data::state::State};

pub fn load_config() -> State {
    fs::read(
        dirs::config_dir()
            .expect("[gui.rs - Sabikui::new]: There should be a config directory")
            .join(constants::CONFIG_DIRECTORY)
            .join(constants::CONFIG_FILE),
    )
    .ok()
    .and_then(|file| toml::from_slice(&file).ok())
    .unwrap_or_default()
}

pub fn save_config(state: &State) {
    let config_folder = dirs::config_dir()
        .expect("[gui.rs - Sabikui::drop]: There should be a config directory")
        .join(constants::CONFIG_DIRECTORY);
    if !config_folder.exists() {
        fs::create_dir(&config_folder).unwrap(); // TODO handle errors
    }

    fs::write(
        config_folder.join(constants::CONFIG_FILE),
        toml::to_string_pretty(&state).expect("[gui.rs - Sabikui::drop]: The serialization should not fail"),
    )
    .unwrap(); // TODO handle errors
}
