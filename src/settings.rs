use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

/// Settings for Macronizer
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    /// The keystrokes which, when pressed, stop macro recording or playback.
    pub stop_keystrokes: Vec<String>,
    /// The strategy used for wait intervals between events. Options: "actual", "none", "constant".
    pub wait_strategy: String,
    /// The constant wait time in milliseconds used if wait_strategy is "constant".
    pub constant_wait_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            stop_keystrokes: vec!["Escape".to_string()],
            wait_strategy: "constant".to_string(),
            constant_wait_ms: 100,
        }
    }
}

/// Loads settings from ~/.config/macronizer/settings.toml.
/// If the settings file does not exist, it creates one with default values.
pub fn load_settings() -> Settings {
    let config_dir = dirs::config_dir()
        .expect("Unable to retrieve config directory")
        .join("macronizer");
    let settings_path = config_dir.join("settings.toml");
    if !settings_path.exists() {
        fs::create_dir_all(&config_dir).expect("Unable to create config directory");
        let default_settings = Settings::default();
        let toml_str =
            toml::to_string(&default_settings).expect("Failed to serialize default settings");
        let mut file = fs::File::create(&settings_path).expect("Failed to create settings file");
        file.write_all(toml_str.as_bytes())
            .expect("Failed to write default settings");
        return default_settings;
    }
    let contents = fs::read_to_string(&settings_path).expect("Failed to read settings file");
    toml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Failed to parse settings file, using default settings");
        Settings::default()
    })
}
