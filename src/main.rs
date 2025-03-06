use clap::{Arg, Command};
use rdev::{listen, simulate, Event, EventType};
use std::sync::Mutex;
use std::{fs, path::PathBuf};
use std::{thread, time};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MacroEvent {
    event_type: String,
    details: String,
}

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

    let macro_events = Mutex::new(Vec::new()); // Store events during recording

    match matches.subcommand() {
        Some(("record", sub_m)) => {
            if let Some(name) = sub_m.value_of("name") {
                println!("Starting to record macro: {}", name);
                record_macro(name, &macro_events);
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
                run_macro(name, repeat);
            }
        }
        _ => println!("No valid subcommand was used"),
    }
}

fn record_macro(name: &str, macro_events: &Mutex<Vec<MacroEvent>>) {
    // Perform a 3 second countdown
    println!("Recording will start in 3 seconds...");
    thread::sleep(time::Duration::from_secs(3));

    // Callback function to handle events
    let callback = |event: Event| {
        if let Ok(mut events) = macro_events.lock() {
            let macro_event = match event.event_type {
                EventType::KeyPress(key) => Some(MacroEvent {
                    event_type: "KeyPress".to_string(),
                    details: format!("{:?}", key),
                }),
                EventType::KeyRelease(key) => Some(MacroEvent {
                    event_type: "KeyRelease".to_string(),
                    details: format!("{:?}", key),
                }),
                EventType::MouseMove { x, y } => Some(MacroEvent {
                    event_type: "MouseMove".to_string(),
                    details: format!("x: {}, y: {}", x, y),
                }),
                EventType::ButtonPress(button) => Some(MacroEvent {
                    event_type: "ButtonPress".to_string(),
                    details: format!("{:?}", button),
                }),
                EventType::ButtonRelease(button) => Some(MacroEvent {
                    event_type: "ButtonRelease".to_string(),
                    details: format!("{:?}", button),
                }),
                _ => None,
            };

            if let Some(m_event) = macro_event {
                events.push(m_event);
            }
        }
    };

    // Start listening to events
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }

    // Save the recorded events to a file
    let path: PathBuf = [
        dirs::config_dir().unwrap().as_path(),
        PathBuf::from("macronizer/macros/").as_path(),
        PathBuf::from(format!("{}.toml", name)).as_path(),
    ]
    .iter()
    .collect();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let events = macro_events.lock().unwrap();
    let toml = toml::to_string(&*events).unwrap();
    fs::write(path, toml).unwrap();
}

fn run_macro(name: &str, repeat: usize) {
    let path: PathBuf = [
        dirs::config_dir().unwrap().as_path(),
        PathBuf::from("macronizer/macros/").as_path(),
        PathBuf::from(format!("{}.toml", name)).as_path(),
    ]
    .iter()
    .collect();

    if let Ok(contents) = fs::read_to_string(path) {
        let macro_events: Vec<MacroEvent> = toml::from_str(&contents).unwrap();
        for _ in 0..repeat {
            for m_event in &macro_events {
                // Simulate event based on type
                let event_type = match m_event.event_type.as_str() {
                    "KeyPress" => Some(EventType::KeyPress(rdev::Key::Unknown)),
                    "KeyRelease" => Some(EventType::KeyRelease(rdev::Key::Unknown)),
                    "MouseMove" => Some(EventType::MouseMove { x: 0.0, y: 0.0 }),
                    "ButtonPress" => Some(EventType::ButtonPress(rdev::Button::Unknown)),
                    "ButtonRelease" => Some(EventType::ButtonRelease(rdev::Button::Unknown)),
                    _ => None,
                };

                if let Some(e_type) = event_type {
                    simulate(&e_type).unwrap();
                }
            }
        }
    } else {
        println!("Could not find macro with name: {}", name);
    }
}
