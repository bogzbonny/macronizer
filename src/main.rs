use clap::{Arg, Command};

fn main() {
    let matches = Command::new("macronizer")
        .version("0.1.0")
        .author("Your Name")
        .about("Records and plays back keyboard and mouse events")
        .subcommand(
            Command::new("record").about("Record a macro").arg(
                Arg::new("name")
                    .help("Name of the macro")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("run")
                .about("Run a macro")
                .arg(
                    Arg::new("name")
                        .help("Name of the macro")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("number")
                        .help("Number of times to repeat the macro")
                        .required(false)
                        .index(2),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("record", sub_m)) => {
            if let Some(name) = sub_m.value_of("name") {
                println!("Starting to record macro: {}", name);
                // Here: Add logic for recording
            }
        }
        Some(("run", sub_m)) => {
            if let Some(name) = sub_m.value_of("name") {
                let repeat = sub_m
                    .value_of("number")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .unwrap_or(1);
                println!("Running macro: {}, {} times", name, repeat);
                // Here: Add logic for playing back
            }
        }
        _ => println!("No valid subcommand was used"),
    }
}
