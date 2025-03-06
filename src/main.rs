use clap::{App, Arg, SubCommand};
use rdev::{listen, Event, EventType};
use std::fs;
use std::path::PathBuf;
use std::{thread, time};

// Define a trait for event listening that can be mocked
trait EventListener {
    fn start(&self, callback: impl Fn(Event) + 'static + Send);
}

// Implement the trait for the real rdev usage
struct RdevListener;

impl EventListener for RdevListener {
    fn start(&self, callback: impl Fn(Event) + 'static + Send) {
        thread::spawn(move || {
            if let Err(error) = listen(callback) {
                println!("Error: {:?}", error);
            }
        });
    }
}

fn start_recording(name: &str, event_listener: &impl EventListener) {
    println!("Recording macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
    let file_path = config_dir.join(format!("{}.toml", name));

    let mut recorded_events = Vec::new();

    let callback = move |event: Event| {
        let recorded_event = match event.event_type {
            EventType::KeyPress(key) => RecordedEvent {
                event_type: "KeyPress".to_string(),
                key: Some(format!("{:?}", key)),
                button: None,
                position: None,
            },
            EventType::KeyRelease(key) => RecordedEvent {
                event_type: "KeyRelease".to_string(),
                key: Some(format!("{:?}", key)),
                button: None,
                position: None,
            },
            EventType::ButtonPress(button) => RecordedEvent {
                event_type: "ButtonPress".to_string(),
                key: None,
                button: Some(format!("{:?}", button)),
                position: None,
            },
            EventType::ButtonRelease(button) => RecordedEvent {
                event_type: "ButtonRelease".to_string(),
                key: None,
                button: Some(format!("{:?}", button)),
                position: None,
            },
            EventType::MouseMove { x, y } => RecordedEvent {
                event_type: "MouseMove".to_string(),
                key: None,
                button: None,
                position: Some((x, y)),
            },
            _ => return,
        };
        recorded_events.push(recorded_event);
    };

    // Use trait-based listener
    event_listener.start(callback);

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

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => println!("Key Press: {:?}", key),
        EventType::KeyRelease(key) => println!("Key Release: {:?}", key),
        EventType::ButtonPress(button) => println!("Button Press: {:?}", button),
        EventType::ButtonRelease(button) => println!("Button Release: {:?}", button),
        EventType::MouseMove { x, y } => println!("Mouse Move: ({}, {})", x, y),
        _ => (),
    }
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
            start_recording(name, &RdevListener);
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
                "KeyPress" => println!("Simulating KeyPress: {:?}", event.key),
                "KeyRelease" => println!("Simulating KeyRelease: {:?}", event.key),
                "ButtonPress" => println!("Simulating ButtonPress: {:?}", event.button),
                "ButtonRelease" => println!("Simulating ButtonRelease: {:?}", event.button),
                "MouseMove" => println!("Simulating MouseMove to: {:?}", event.position),
                _ => (),
            }
        }
    }
}
