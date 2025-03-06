use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

// Mock trait for local event simulation
pub trait EventListener {
    fn simulate(&self, callback: impl FnMut(RecordedEvent) + 'static + Send);
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

    pub fn simulate_event(&self, event: RecordedEvent) {
        self.triggered_events.lock().unwrap().push(event);
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
}

pub fn start_recording(name: &str, event_listener: &impl EventListener) {
    println!("Recording macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
    let file_path = config_dir.join(format!("{}.toml", name));

    let recorded_events = Arc::new(Mutex::new(Vec::new()));
    let recorded_events_clone = Arc::clone(&recorded_events);

    let callback = move |event: RecordedEvent| {
        let mut events = recorded_events_clone.lock().unwrap();
        events.push(event);
    };
    event_listener.simulate(callback);
    thread::sleep(time::Duration::from_secs(3));

    let events = recorded_events.lock().unwrap();
    let toml_string = toml::to_string(&*events).expect("Failed to serialize events");
    fs::write(file_path, toml_string).expect("Failed to save macro file");
}

pub struct RecordedEvent {
    event_type: String,
    key: Option<String>,
    button: Option<String>,
    position: Option<(f64, f64)>,
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
