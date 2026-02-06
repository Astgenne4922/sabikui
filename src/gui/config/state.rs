use crate::gui::{constants, data::state::State};

pub fn load_config() -> State {
    std::fs::read(
        dirs::config_dir()
            .expect("There should be a config directory")
            .join(constants::CONFIG_DIRECTORY)
            .join(constants::CONFIG_FILE),
    )
    .ok()
    .and_then(|file| toml::from_slice(&file).ok())
    .unwrap_or_default()
}

pub fn save_config(state: &State) {
    std::fs::write(
        dirs::config_dir()
            .expect("There should be a config directory")
            .join(constants::CONFIG_DIRECTORY)
            .join(constants::CONFIG_FILE),
        toml::to_string_pretty(&state).expect("The serialization should not fail"),
    )
    .unwrap(); // TODO handle errors
}
