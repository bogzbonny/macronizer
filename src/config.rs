use {
    anyhow::{Context, Error},
    config::Config as ConfigRs,
    serde::{Deserialize, Serialize},
    std::fs,
    std::path::PathBuf,
};

pub const DEFAULT_CONFIG_PATH: &str = "";

/// Config for Macronizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_stop_keystrokes")]
    /// The keystrokes which, when pressed, stop macro recording or playback.
    pub stop_keystrokes: Vec<char>,

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
            stop_keystrokes: vec!['\x1b', '\x1b', '\x1b'], // esc, esc, esc
            wait_strategy: WaitStrategy::ConstantMS(100),
            countdown_seconds: 3,
            recording_initial_wait_ms: 100,
        }
    }
}

fn default_stop_keystrokes() -> Vec<char> {
    vec!['\x1b', '\x1b', '\x1b'] // esc, esc, esc
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
pub enum WaitStrategy {
    /// record actual time between events and put that in there
    Actual,
    /// wait constant ms between events
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

pub fn settings_path() -> PathBuf {
    let p = macronizer_path();
    p.join("settings.toml");
    p
}

pub fn macros_path() -> PathBuf {
    let p = macronizer_path();
    p.join("macros");
    p
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let home = home::home_dir().expect("Failed to find home directory");
        // check for .config/macronizer/ folder and create it if it doesn't exist
        let config_dir = home.join(".config/macronizer");
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let settings_path = config_dir.join("settings.toml");

        let config = ConfigRs::builder()
            .add_source(config::File::with_name(
                settings_path.to_str().expect("Invalid path"),
            ))
            .build()
            .context("Failed to build configuration")?;

        Ok(config.try_deserialize()?)
    }
}
