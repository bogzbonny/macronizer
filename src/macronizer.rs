use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};
use std::{thread, time};

// Mock trait for local event simulation
pub trait EventListener {
    fn simulate(&self, callback: impl FnMut(RecordedEvent) + 'static + Send);
    fn simulate_event(&self, event: RecordedEvent);
}

pub struct MockListener {
    triggered_events: Mutex<Vec<RecordedEvent>>,
    wait_condition_met: Mutex<bool>,
}

impl MockListener {
    pub fn new() -> Self {
        MockListener {
            triggered_events: Mutex::new(Vec::new()),
            wait_condition_met: Mutex::new(false),
        }
    }

    pub fn was_event_triggered(&self, event_type: &str, key: &str) -> bool {
        let events = self.triggered_events.lock().unwrap();
        events
            .iter()
            .any(|e| e.event_type == event_type && e.key.as_deref() == Some(key))
    }

    pub fn was_wait_condition_met(&self) -> bool {
        *self.wait_condition_met.lock().unwrap()
    }

    pub fn get_triggered_events_len(&self) -> usize {
        let events = self.triggered_events.lock().unwrap();
        events.len()
    }
}

impl EventListener for MockListener {
    fn simulate(&self, mut callback: impl FnMut(RecordedEvent) + 'static + Send) {
        thread::spawn(move || {
            let key_press_event = RecordedEvent {
                event_type: "KeyPress".to_string(),
                key: Some("MockKey".to_string()),
                button: None,
                position: None,
            };

            callback(key_press_event);
        });
    }

    fn simulate_event(&self, event: RecordedEvent) {
        self.triggered_events.lock().unwrap().push(event);
    }
}

pub fn start_recording(name: &str, event_listener: &impl EventListener) {
    println!("Recording macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");

    // Create macros directory if it does not exist
    fs::create_dir_all(&config_dir).expect("Failed to create macros directory");

    let file_path = config_dir.join(format!("{}.toml", name));

    let recorded_events = Arc::new(Mutex::new(Vec::new()));
    let recorded_events_clone = Arc::clone(&recorded_events);

    let callback = move |event: RecordedEvent| {
        let mut events = recorded_events_clone.lock().unwrap();
        events.push(event);
    };
    event_listener.simulate(callback);
    thread::sleep(time::Duration::from_secs(3));

    {
        let events = recorded_events.lock().unwrap();

        // Construct a proper TOML array of tables
        let toml_string = events
            .iter()
            .map(|event| {
                format!(
                    "[[events]]\n{}",
                    toml::to_string_pretty(event).expect("Failed to serialize event")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!(
            "Serialized Correct Events TOML:
{}",
            toml_string
        );

        println!("Saving to path: {:?}", file_path);

        fs::write(file_path, toml_string).expect("Failed to save macro file");
    }
}

pub fn start_playback(name: &str, event_listener: &impl EventListener) {
    println!("Playing back macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");

    // Create macros directory if it does not exist
    fs::create_dir_all(&config_dir).expect("Failed to create macros directory");

    let file_path = config_dir.join(format!("{}.toml", name));

    let contents = fs::read_to_string(file_path).expect("Failed to read macro file");

    // Deserialize directly as a vector of RecordedEvent
    let events: Vec<RecordedEvent> =
        toml::from_str(&contents).expect("Failed to deserialize macro file");

    println!("Deserialized Events: {:?}", events);

    for event in events {
        event_listener.simulate_event(event);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RecordedEvent {
    pub event_type: String,
    pub key: Option<String>,
    pub button: Option<String>,
    pub position: Option<(f64, f64)>,
}

impl RecordedEvent {
    pub fn get_event_type(&self) -> &str {
        &self.event_type
    }

    pub fn get_key(&self) -> Option<&str> {
        self.key.as_deref()
    }
}

pub fn handle_stop_keystroke(listener: &MockListener) {
    listener.simulate_event(RecordedEvent {
        event_type: "Stop".to_string(),
        key: None,
        button: None,
        position: None,
    });
}

pub fn simulate_wait(listener: &MockListener) {
    let mut wait_met = listener.wait_condition_met.lock().unwrap();
    *wait_met = true;
}

pub fn simulate_button_press(listener: &MockListener, button: &str) {
    listener.simulate_event(RecordedEvent {
        event_type: "ButtonPress".to_string(),
        key: None,
        button: Some(button.to_string()),
        position: None,
    });
}

pub fn simulate_button_release(listener: &MockListener, button: &str) {
    listener.simulate_event(RecordedEvent {
        event_type: "ButtonRelease".to_string(),
        key: None,
        button: Some(button.to_string()),
        position: None,
    });
}

pub fn simulate_mouse_movement(listener: &MockListener, x: i32, y: i32) {
    listener.simulate_event(RecordedEvent {
        event_type: "MouseMove".to_string(),
        key: None,
        button: None,
        position: Some((x as f64, y as f64)),
    });
}
