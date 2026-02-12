fn main() {
    let config_folder = dirs::config_dir()
        .expect("There should be a config directory")
        .join("sabikui");
    if !config_folder.exists() {
        std::fs::create_dir(&config_folder).expect("The config folder creation should not fail");
    }

    let alg_dir = config_folder.join("hash_functions");
    if !alg_dir.exists() {
        std::fs::create_dir(&alg_dir).expect("The hash folder creation should not fail");
    }

    let theme_dir = config_folder.join("theme");
    if !theme_dir.exists() {
        std::fs::create_dir(&theme_dir).expect("The theme folder creation should not fail");
    }

    let (dark_theme, light_theme) = (theme_dir.join("dark.toml"), theme_dir.join("light.toml"));
    if !dark_theme.exists() {
        std::fs::write(dark_theme, include_bytes!("assets/theme/dark.toml"))
            .expect("There should be no errors when writing the default themes");
    }
    if !light_theme.exists() {
        std::fs::write(light_theme, include_bytes!("assets/theme/light.toml"))
            .expect("There should be no errors when writing the default themes");
    }
}
