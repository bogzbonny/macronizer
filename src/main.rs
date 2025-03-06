use clap::{Arg, Command};
use std::fs;
use std::thread;
use std::time;

mod macronizer;

use macronizer::{start_playback, start_recording, MockListener, RdevListener};

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
stop_keystrokes = [\"ControlLeft\", \"ShiftRight\"]

# Default wait strategy - options: actual, none, constant
wait_strategy = \"constant\"
constant_wait_time = 100  # milliseconds
"#;
        fs::write(&settings_file, default_settings).expect("Failed to write default settings");
    }

    // Setup CLI with clap
    let matches = Command::new("macronizer")
        .version("0.1.0")
        .author("Author Name <email@example.com>")
        .about("Records and plays back system-wide keyboard and mouse events")
        .subcommand(
            Command::new("record")
                .about("Starts recording a macro")
                .arg(
                    Arg::new("name")
                        .help("Name of the macro to record")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("real")
                        .long("real")
                        .help("Use real event listener")
                        .takes_value(false),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Runs a recorded macro")
                .arg(
                    Arg::new("name")
                        .help("Name of the macro to run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("number")
                        .help("Number of times to repeat the macro")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("real")
                        .long("real")
                        .help("Use real event listener")
                        .takes_value(false),
                ),
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("record", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let use_real = sub_m.get_flag("real");
            println!("Starting to record macro: {}", name);
            if use_real {
                start_recording(name, &RdevListener::new());
            } else {
                start_recording(name, &MockListener::new());
            }
        }
        Some(("run", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let repeat = sub_m
                .get_one::<String>("number")
                .map_or("1".to_string(), |v| v.to_string())
                .parse::<u32>()
                .unwrap();
            let use_real = sub_m.get_flag("real");
            println!("Running macro: {} for {} times", name, repeat);
            for _ in 0..repeat {
                if use_real {
                    start_playback(name, &RdevListener::new());
                } else {
                    start_playback(name, &MockListener::new());
                }
            }
        }
        _ => {}
    }
}
