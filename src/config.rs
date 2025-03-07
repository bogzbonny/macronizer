use {
    anyhow::{Context, Error},
    config::Config as ConfigRs,
    serde::{Deserialize, Serialize},
    std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    },
};

/// Config for Macronizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_stop_keystrokes")]
    /// The keystrokes which, when pressed, stop macro recording or playback.
    pub stop_keystrokes: Vec<rdev::Key>,

    #[serde(default)]
    pub wait_strategy: WaitStrategy,

    #[serde(default = "default_countdown_seconds")]
    /// number of seconds to count down for.
    pub countdown_seconds: u64,

    /// whether to record mouse moves that occur without a button being pressed
    /// down
    #[serde(default = "default_record_non_drag_mouse_moves")]
    pub record_non_drag_mouse_moves: bool,

    /// Initial wait time is never actually recorded, all event recordings only on the first
    /// true mouse/keyboard event. This is to take the pressure off.
    /// This value will be added to the very beginning of each recording to ensure that the
    /// recording can start with a delay.
    #[serde(default = "default_recording_initial_wait_ms")]
    pub recording_initial_wait_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            stop_keystrokes: vec![rdev::Key::Escape, rdev::Key::Escape, rdev::Key::Escape],
            wait_strategy: WaitStrategy::ConstantMS(100),
            countdown_seconds: 3,
            record_non_drag_mouse_moves: false,
            recording_initial_wait_ms: 100,
        }
    }
}

fn default_stop_keystrokes() -> Vec<rdev::Key> {
    vec![rdev::Key::Escape, rdev::Key::Escape, rdev::Key::Escape]
}
fn default_countdown_seconds() -> u64 {
    3
}
fn default_record_non_drag_mouse_moves() -> bool {
    false
}
fn default_recording_initial_wait_ms() -> u64 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitStrategy {
    /// record actual time between events and put that in there
    Actual,
    /// wait constant ms between releases events
    ConstantMS(u64),
}

impl Default for WaitStrategy {
    fn default() -> Self {
        WaitStrategy::ConstantMS(100)
    }
}

pub fn macronizer_path() -> PathBuf {
    let home = home::home_dir().expect("Failed to find home directory");
    home.join(".config/macronizer")
}

pub fn macros_path() -> PathBuf {
    let p = macronizer_path();
    p.join("macros")
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        // check for .config/macronizer/ folder and create it if it doesn't exist
        let config_dir = macronizer_path();
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let settings_path = config_dir.join("settings.toml");

        // create the settings.toml file if it doesn't exist
        if !settings_path.exists() {
            let mut file = File::create(&settings_path)?;
            let default_config = Config::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            file.write_all(toml_string.as_bytes())?;
        }

        let config = ConfigRs::builder()
            .add_source(config::File::with_name(
                settings_path.to_str().expect("Invalid path"),
            ))
            .build()
            .context("Failed to build configuration")?;

        Ok(config.try_deserialize()?)
    }
}
