mod config;
mod macronizer;

use {
    crate::config::Config,
    anyhow::Error,
    clap::{Parser, Subcommand},
    macronizer::*,
    std::{fs, thread, time::Duration},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Starts recording a macro
    Rec {
        /// Name of the macro to record
        name: String,

        /// Add a description to the macro
        #[arg(short, long, default_value = "add a description")]
        desc: String,

        /// Allow overwriting existing macro
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Runs a recorded macro
    Run {
        /// Name of the macro to run
        name: String,

        /// Number of times to repeat the macro
        #[arg(short = 'n', long = "repeat", default_value_t = 1)]
        repeat: usize,
    },
    /// List all recorded macros
    Ls,
    /// Remove the specified macro
    Rm {
        /// Name of the macro to remove
        name: String,
    },
}

fn main() -> Result<(), Error> {
    let cfg = Config::load()?;
    let cli = Cli::parse();

    // Handle subcommands
    match &cli.command {
        Commands::Rec {
            name,
            desc,
            overwrite,
        } => {
            if !*overwrite {
                // if overwrite is not set, check if file exists and prevent overwriting
                let macros_dir = config::macros_path();
                let file_path = macros_dir.join(format!("{}.toml", name));
                if file_path.exists() {
                    eprintln!("macro \"{name}\" already exists, use --overwrite to overwrite");
                    return Ok(());
                }
            }

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
            record(&cfg, name.to_string(), desc.to_string());
        }
        Commands::Run { name, repeat } => {
            let macros_dir = config::macros_path();
            let file_path = macros_dir.join(format!("{}.toml", name));
            if !file_path.exists() {
                eprintln!("macro \"{name}\" not found");
                return Ok(());
            }

            println!("Running macro: {} for {} time(s)", name, repeat);
            let secs = cfg.countdown_seconds;
            println!("Playback starts in...");
            for i in (1..=secs).rev() {
                println!("{}...", i);
                thread::sleep(Duration::from_millis(950));
            }
            println!("Begin!");
            let middle_e_hz = 329;
            let a_bit_more_than_a_second_and_a_half_ms = 100;
            actually_beep::beep_with_hz_and_millis(
                middle_e_hz,
                a_bit_more_than_a_second_and_a_half_ms,
            )
            .unwrap();
            for _ in 0..*repeat {
                start_playback(&cfg, name);
            }
        }
        Commands::Ls => {
            let macros_dir = config::macros_path();
            // write all files in the directory to stdout but not the toml extension
            for entry in fs::read_dir(macros_dir).expect("Failed to read macros directory") {
                let entry = entry.expect("Failed to read macros directory entry");
                let path = entry.path();
                if path.is_file() && path.extension().is_some() {
                    // get the description from the toml file
                    let contents = fs::read_to_string(&path).expect("Failed to read file");
                    let evs: Macro = match toml::from_str(&contents) {
                        Ok(evs) => evs,
                        Err(e) => {
                            println!("Failed to deserialize macro file: {:?}", e);
                            return Ok(());
                        }
                    };
                    let description = evs.description;
                    let name = path
                        .file_stem()
                        .expect("Failed to get file stem")
                        .to_str()
                        .expect("Failed to convert file stem to str");

                    print!("{name:<27} - ");

                    // print description with line breaks
                    let mut lines = description.lines();
                    if let Some(first_line) = lines.next() {
                        println!("{}", first_line); // Print the first line after the first field
                    }
                    for line in lines {
                        println!("{:<30}{line}", "");
                    }
                }
            }
        }
        Commands::Rm { name } => {
            let macros_dir = config::macros_path();
            let file_path = macros_dir.join(format!("{}.toml", name));
            if !file_path.exists() {
                eprintln!("macro \"{name}\" not found");
                return Ok(());
            }
            fs::remove_file(&file_path).expect("Failed to remove existing macro file");
        }
    }
    Ok(())
}
