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
        events.iter().any(|e| {
            e.event_type == event_type && e.key.as_deref().unwrap_or("") == key
        })
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
        let key_press_event = RecordedEvent {
            event_type: "KeyPress".to_string(),
            key: Some("MockKey".to_string()),
            button: None,
            position: None,
        };

        callback(key_press_event);
    }

    fn simulate_event(&self, event: RecordedEvent) {
        self.triggered_events.lock().unwrap().push(event);
    }
}

pub struct RecordedEvents {
    pub events: Vec<RecordedEvent>,
}

impl<'de> Deserialize<'de> for RecordedEvents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper {
            events: Vec<RecordedEvent>,
        }

        let wrapper = Wrapper::deserialize(deserializer)?;
        Ok(RecordedEvents {
            events: wrapper.events,
        })
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
        println!("Recording event: {:?}", event); 
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
