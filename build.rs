fn main() {
    let config_folder = dirs::config_dir()
        .expect("There should be a config directory")
        .join("sabikui");
    if !config_folder.exists() {
        std::fs::create_dir(&config_folder).unwrap(); // TODO handle errors
    }

    let alg_dir = config_folder.join("hash_functions");
    if !alg_dir.exists() {
        std::fs::create_dir(&alg_dir).unwrap(); // TODO handle errors
    }

    let theme_dir = config_folder.join("theme");
    if !theme_dir.exists() {
        std::fs::create_dir(&theme_dir).unwrap(); // TODO handle errors
    }

    let (dark_theme, light_theme) = (theme_dir.join("dark.toml"), theme_dir.join("light.toml"));
    if !dark_theme.exists() {
        std::fs::write(dark_theme, include_bytes!("assets/theme/dark.toml")).unwrap(); // TODO handle errors
    }
    if !light_theme.exists() {
        std::fs::write(light_theme, include_bytes!("assets/theme/light.toml")).unwrap(); // TODO handle errors
    }
}
