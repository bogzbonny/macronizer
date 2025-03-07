mod config;
mod macronizer;

use {
    crate::config::Config,
    anyhow::Error,
    clap::Command,
    macronizer::*,
    std::{thread, time::Duration},
};

fn main() -> Result<(), Error> {
    let cfg = Config::load()?;

    let matches = Command::new("macronizer")
        .version("0.1.0")
        .author("Author Name <email@example.com>")
        .about("Records and plays back system-wide keyboard and mouse events")
        .subcommand(
            Command::new("rec").about("Starts recording a macro").arg(
                // Name of macro
                clap::Arg::new("name")
                    .help("Name of the macro to record")
                    .required(true)
                    .index(1),
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
                ),
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("rec", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let secs = cfg.countdown_seconds;
            println!(
                "Beginning recording, default mapping for ending the recording is Esc+Esc+Esc"
            );
            println!("Recording starts in...");
            for i in (1..=secs).rev() {
                println!("{}...", i);
                thread::sleep(Duration::from_millis(950));
            }
            println!("Start!");
            let middle_e_hz = 329;
            let a_bit_more_than_a_second_and_a_half_ms = 100;
            actually_beep::beep_with_hz_and_millis(
                middle_e_hz,
                a_bit_more_than_a_second_and_a_half_ms,
            )
            .unwrap();
            record(&cfg, name.to_string());
        }
        Some(("run", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let repeat = sub_m
                .get_one::<String>("number")
                .map_or("1".to_string(), |v| v.to_string())
                .parse::<u32>()
                .unwrap();
            println!("Running macro: {} for {} time(s)", name, repeat);
            for _ in 0..repeat {
                start_playback(&cfg, name);
            }
        }
        _ => {}
    }
    Ok(())
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use {
//        macronizer::{RdevListener, RecordedEvents},
//        std::fs,
//    };

//    #[test]
//    fn test_record_event() {
//        // Remove existing test_macro file if it exists
//        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
//        let file_path = config_dir.join("test_macro.toml");
//        if file_path.exists() {
//            fs::remove_file(&file_path).expect("Failed to remove existing macro file");
//        }

//        // Use default MockListener for testing
//        let mock_listener = MockListener::default();

//        // Call the recording function passing the mock listener
//        start_recording("test_macro", &mock_listener);

//        // Read and assert the contents of the file
//        let contents = fs::read_to_string(file_path).expect("Failed to read macro file");
//        let recorded: RecordedEvents =
//            toml::from_str(&contents).expect("Failed to deserialize macro file");

//        assert_eq!(recorded.events.len(), 3); // Expect KeyPress, ButtonPress, and MouseMove events
//        assert_eq!(recorded.events[0].event_type, "KeyPress");
//        assert_eq!(recorded.events[0].key.as_deref(), Some("MockKey"));
//        assert_eq!(recorded.events[1].event_type, "ButtonPress");
//        assert_eq!(recorded.events[1].button.as_deref(), Some("Button1"));
//    }
//}
