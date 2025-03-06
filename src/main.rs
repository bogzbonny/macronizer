use clap::{Arg, ArgAction, Command};
use std::fs;
use std::{thread, time::Duration};

mod macronizer;
mod settings;

use macronizer::{
    start_playback, start_recording, MockListener, RdevListener, RecordedEvent, RecordedEvents,
};
use settings::load_settings;

// Import tinyaudio for playing the bell sound
use tinyaudio::{Player, Sound};

/// Plays a short bell noise using tinyaudio.
fn play_bell() {
    // Parameters for the bell sound
    let sample_rate = 44100;
    let frequency = 880.0; // Higher pitch for bell-like sound
    let duration_secs = 0.3; // 0.3 second duration
    let num_samples = (sample_rate as f32 * duration_secs) as usize;

    // Generate a sine wave
    let samples: Vec<f32> = (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * std::f32::consts::PI * frequency * t).sin()
        })
        .collect();

    // Create a Sound from the generated samples
    let sound = Sound::from_pcm(samples, sample_rate as u32, 1);

    // Create a default audio player and play the sound
    let player = Player::default();
    player.play(sound);
}

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
            // Play bell sound to indicate recording has begun
            play_bell();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macronizer::{MockListener, RecordedEvent, RecordedEvents};

    #[test]
    fn test_record_event() {
        // MockListener instantiation simulating event handling
        let mock_listener = MockListener::default();

        // Call the recording function passing the mock listener
        start_recording("test_macro", &mock_listener);

        // Validate that the recordings are saved
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        let file_path = config_dir.join("test_macro.toml");

        // Read and assert the contents of the file
        let contents = fs::read_to_string(file_path).expect("Failed to read macro file");
        let recorded: RecordedEvents =
            toml::from_str(&contents).expect("Failed to deserialize macro file");

        assert_eq!(recorded.events.len(), 3); // Expect KeyPress, ButtonPress, and MouseMove events
        assert_eq!(recorded.events[0].event_type, "KeyPress");
        assert_eq!(recorded.events[0].key.as_deref(), Some("MockKey"));
        assert_eq!(recorded.events[1].event_type, "ButtonPress");
        assert_eq!(recorded.events[1].button.as_deref(), Some("Button1"));
    }

    // Optional: Add more tests to cover macro playback and additional scenarios
}
