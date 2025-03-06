use clap::{Arg, Command};
use rdev;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{thread, time};

// Mock trait for local event simulation
trait EventListener {
    fn simulate(&self, callback: impl Fn(RecordedEvent) + 'static + Send);
}

struct MockListener;

impl EventListener for MockListener {
    fn simulate(&self, callback: impl Fn(RecordedEvent) + 'static + Send) {
        thread::spawn(move || {
            let mock_event = RecordedEvent {
                event_type: "KeyPress".to_string(),
                key: Some("MockKey".to_string()),
                button: None,
                position: None,
            };
            callback(mock_event);
        });
    }
}

fn start_recording(name: &str, event_listener: &impl EventListener) {
    println!("Recording macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
    let file_path = config_dir.join(format!("{}.toml", name));

    let mut recorded_events = Vec::new();

    let callback = move |event: RecordedEvent| {
        recorded_events.push(event);
    };

    // Use mock listener
    event_listener.simulate(callback);

    // Simulate recording duration or waiting before starting
    thread::sleep(time::Duration::from_secs(3));

    // Serialize and save events
    let toml_string = toml::to_string(&recorded_events).expect("Failed to serialize events");
    fs::write(file_path, toml_string).expect("Failed to save macro file");
}

#[derive(Serialize, Deserialize, Debug)]
struct RecordedEvent {
    event_type: String,
    key: Option<String>,
    button: Option<String>,
    position: Option<(f64, f64)>,
}

fn main() {
    // Establish configuration directories
    let config_dir = dirs::config_dir().unwrap().join("macronizer");
    let macros_dir = config_dir.join("macros");
    let settings_file = config_dir.join("settings.toml");

    // Ensure directories and files are created
    fs::create_dir_all(&macros_dir).expect("Failed to create macros directory");
    if !settings_file.exists() {
        fs::write(&settings_file, "").expect("Failed to create settings file");
    }
    // Create settings file with defaults if it doesn't exist or is empty
    if !settings_file.exists()
        || fs::read_to_string(&settings_file)
            .unwrap()
            .trim()
            .is_empty()
    {
        let default_settings = r#"# Default stop recording/playback keystrokes
stop_keystrokes = ["ControlLeft", "ShiftRight"]

# Default wait strategy - options: actual, none, constant
wait_strategy = "constant"
constant_wait_time = 100  # milliseconds
"#;

        fs::write(&settings_file, default_settings).expect("Failed to write default settings");
    }

    // Setup CLI with clap
    let matches = App::new("macronizer")
        .version("0.1.0")
        .author("Author Name <email@example.com>")
        .about("Records and plays back system-wide keyboard and mouse events")
        .subcommand(
            SubCommand::with_name("record")
                .about("Starts recording a macro")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the macro to record")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a recorded macro")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the macro to run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("number")
                        .help("Number of times to repeat the macro")
                        .required(false)
                        .index(2),
                ),
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        ("record", Some(sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            println!("Starting to record macro: {}", name);
            start_recording(name, &MockListener);
        }
        ("run", Some(sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let repeat = sub_m
                .value_of("number")
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap();
            println!("Running macro: {} for {} times", name, repeat);

            let macro_player = MacroPlayer::new(name);
            for _ in 0..repeat {
                macro_player.play();
            }
        }
        _ => {}
    }
}

struct MacroPlayer {
    events: Vec<RecordedEvent>,
}

impl MacroPlayer {
    fn new(name: &str) -> Self {
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        let file_path = config_dir.join(format!("{}.toml", name));

        let contents = fs::read_to_string(file_path).expect("Failed to read macro file");
        let events: Vec<RecordedEvent> =
            toml::from_str(&contents).expect("Failed to deserialize macro file");

        MacroPlayer { events }
    }

    fn play(&self) {
        for event in &self.events {
            match event.event_type.as_str() {
                "KeyPress" => {
                    if let Some(key_str) = &event.key {
                        if let Ok(key) = key_str.parse::<rdev::Key>() {
                            rdev::simulate(&rdev::EventType::KeyPress(key))
                                .unwrap_or_else(|e| println!("Error simulating KeyPress: {:?}", e));
                            rdev::simulate(&rdev::EventType::KeyRelease(key)).unwrap_or_else(|e| {
                                println!("Error simulating KeyRelease: {:?}", e)
                            });
                        }
                    }
                }
                "KeyRelease" => {
                    if let Some(key_str) = &event.key {
                        if let Ok(key) = key_str.parse::<rdev::Key>() {
                            rdev::simulate(&rdev::EventType::KeyRelease(key)).unwrap_or_else(|e| {
                                println!("Error simulating KeyRelease: {:?}", e)
                            });
                        }
                    }
                }
                "ButtonPress" => {
                    if let Some(button_str) = &event.button {
                        if let Ok(button) = button_str.parse::<rdev::Button>() {
                            rdev::simulate(&rdev::EventType::ButtonPress(button)).unwrap_or_else(
                                |e| println!("Error simulating ButtonPress: {:?}", e),
                            );
                        }
                    }
                }
                "ButtonRelease" => {
                    if let Some(button_str) = &event.button {
                        if let Ok(button) = button_str.parse::<rdev::Button>() {
                            rdev::simulate(&rdev::EventType::ButtonRelease(button)).unwrap_or_else(
                                |e| println!("Error simulating ButtonRelease: {:?}", e),
                            );
                        }
                    }
                }
                "MouseMove" => {
                    if let Some((x, y)) = event.position {
                        rdev::simulate(&rdev::EventType::MouseMove { x, y })
                            .unwrap_or_else(|e| println!("Error simulating MouseMove: {:?}", e));
                    }
                }
                _ => (),
            }
        }
    }
}
