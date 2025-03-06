use clap::{Arg, ArgAction, Command};
use std::fs;
use std::{thread, time::Duration};

mod macronizer;
mod settings;

use macronizer::{start_playback, start_recording, MockListener, RdevListener};
use settings::load_settings;

fn main() {
    // Establish configuration directories
    let config_dir = dirs::config_dir().unwrap().join("macronizer");
    let macros_dir = config_dir.join("macros");
    fs::create_dir_all(&macros_dir).expect("Failed to create macros directory");

    // Load settings (this will create the file with defaults if needed)
    let settings = load_settings();
    println!("Loaded settings: {:?}", settings);

    // Setup CLI with clap
    let matches = Command::new("macronizer")
        .version("0.1.0")
        .author("Author Name <email@example.com>")
        .about("Records and plays back system-wide keyboard and mouse events")
        .subcommand(
            Command::new("record")
                .about("Starts recording a macro")
                .arg(
                    // Name of macro
                    clap::Arg::new("name")
                        .help("Name of the macro to record")
                        .required(true)
                        .index(1),
                )
                .arg(
                    clap::Arg::new("real")
                        .long("real")
                        .help("Use real event listener")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Runs a recorded macro")
                .arg(
                    clap::Arg::new("name")
                        .help("Name of the macro to run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    clap::Arg::new("number")
                        .help("Number of times to repeat the macro")
                        .required(false)
                        .index(2),
                )
                .arg(
                    clap::Arg::new("real")
                        .long("real")
                        .help("Use real event listener")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("record", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let use_real = sub_m.get_flag("real");
            println!("Preparing to record macro: {}", name);
            // 3-second countdown before recording starts
            for i in (1..=3).rev() {
                println!("Recording starts in {}...", i);
                thread::sleep(Duration::from_secs(1));
            }
            println!("Recording started!");

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
