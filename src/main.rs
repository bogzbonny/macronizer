use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::PathBuf;

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
            // ToDo: Implement recording functionality
        }
        ("run", Some(sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let repeat = sub_m
                .value_of("number")
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap();
            println!("Running macro: {} for {} times", name, repeat);
            // ToDo: Implement run functionality
        }
        _ => {}
    }
}
