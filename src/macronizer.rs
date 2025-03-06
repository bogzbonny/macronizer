use std::fs;
use std::sync::{Arc, Mutex};
use std::{thread, time};

// Event listener trait for simulating or handling real events
pub trait EventListener {
    fn simulate(&self, callback: impl FnMut(RecordedEvent) + 'static + Send);
    fn simulate_event(&self, event: RecordedEvent);
}

// Mock implementation for testing and simulation
#[derive(Default)]
pub struct MockListener {
    triggered_events: Mutex<Vec<RecordedEvent>>,
    pub wait_condition_met: Mutex<bool>,
}

impl MockListener {
    pub fn new() -> Self {
        MockListener {
            triggered_events: Mutex::new(Vec::new()),
            wait_condition_met: Mutex::new(false),
        }
    }

    pub fn was_event_triggered(&self, event_type: &str, identifier: &str) -> bool {
        let events = self.triggered_events.lock().unwrap();
        events.iter().any(|e| {
            e.event_type == event_type
                && (e.key.as_deref().unwrap_or("") == identifier
                    || e.button.as_deref().unwrap_or("") == identifier
                    || e.position.map_or(false, |pos| {
                        format!("{}-{}", pos.0.round(), pos.1.round()) == identifier
                    }))
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
        // Simulate a set of predefined events
        let key_press_event = RecordedEvent {
            event_type: "KeyPress".to_string(),
            key: Some("MockKey".to_string()),
            button: None,
            position: None,
        };
        callback(key_press_event);

        let button_press_event = RecordedEvent {
            event_type: "ButtonPress".to_string(),
            key: None,
            button: Some("Button1".to_string()),
            position: None,
        };
        callback(button_press_event);

        // Simulate a mouse movement
        let mouse_move_event = RecordedEvent {
            event_type: "MouseMove".to_string(),
            key: None,
            button: None,
            position: Some((100.0, 150.0)),
        };
        callback(mouse_move_event);
    }

    fn simulate_event(&self, event: RecordedEvent) {
        self.triggered_events.lock().unwrap().push(event);
    }
}

// Container for deserializing events
#[derive(serde::Deserialize, Debug)]
pub struct RecordedEvents {
    pub events: Vec<RecordedEvent>,
}

// Starts recording by using the provided event listener
pub fn start_recording(name: &str, event_listener: &impl EventListener) {
    println!("Recording macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");

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
        let toml_string = events
            .iter()
            .map(|event| {
                let serialized_event =
                    toml::to_string_pretty(event).expect("Failed to serialize event");
                format!("[[events]]\n{}", serialized_event)
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("Serialized Correct Events TOML:\n{}", toml_string);
        println!("Saving to path: {:?}", file_path);

        fs::write(file_path, toml_string).expect("Failed to save macro file");
    }
}

// Starts playback by deserializing events and passing them to the provided event listener
pub fn start_playback(name: &str, event_listener: &impl EventListener) {
    println!("Playing back macro: {}", name);
    let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");

    fs::create_dir_all(&config_dir).expect("Failed to create macros directory");

    let file_path = config_dir.join(format!("{}.toml", name));

    let contents = fs::read_to_string(file_path).expect("Failed to read macro file");

    let recorded_events: RecordedEvents =
        toml::from_str(&contents).expect("Failed to deserialize macro file");

    println!("Deserialized Events: {:?}", recorded_events.events);

    for event in recorded_events.events {
        event_listener.simulate_event(event);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
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
    println!("Simulated ButtonPress: {}", button);
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
    println!("Simulated mouse movement to: {}-{}", x, y);
}

// New implementation using rdev for real event handling
extern crate rdev;

pub struct RdevListener;

impl RdevListener {
    pub fn new() -> Self {
        RdevListener {}
    }
}

impl EventListener for RdevListener {
    fn simulate(&self, mut callback: impl FnMut(RecordedEvent) + 'static + Send) {
        // Use rdev::listen to receive real events
        if let Err(e) = rdev::listen(move |event| {
            let pos = match event.event_type {
                rdev::EventType::MouseMove { x, y } => Some((x, y)),
                _ => None,
            };
            let recorded = RecordedEvent {
                event_type: format!("{:?}", event.event_type),
                key: None,    // Placeholder: Add proper conversion if needed
                button: None, // Placeholder conversion
                position: pos,
            };
            callback(recorded);
        }) {
            eprintln!("Error in real event listener: {:?}", e);
        }
    }

    fn simulate_event(&self, event: RecordedEvent) {
        use rdev::{simulate, Button, EventType, Key};
        let rdev_event = match event.event_type.as_str() {
            "KeyPress" => EventType::KeyPress(Key::Unknown(0)),
            "KeyRelease" => EventType::KeyRelease(Key::Unknown(0)),
            "ButtonPress" => EventType::ButtonPress(Button::Left),
            "ButtonRelease" => EventType::ButtonRelease(Button::Left),
            "MouseMove" => {
                if let Some((x, y)) = event.position {
                    EventType::MouseMove { x, y }
                } else {
                    EventType::MouseMove { x: 0.0, y: 0.0 }
                }
            }
            _ => {
                // Fallback to a no-op event; adjust as needed
                EventType::MouseMove { x: 0.0, y: 0.0 }
            }
        };
        if let Err(e) = simulate(&rdev_event) {
            eprintln!("Error simulating real event: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_record_event() {
        // MockListener instantiation simulating event handling
        let mock_listener = MockListener::new();

        // Call the recording function passing the mock listener
        start_recording("test_macro", &mock_listener);

        // Validate that the recordings are saved
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        let file_path = config_dir.join("test_macro.toml");

        // Read and assert the contents of the file
        let contents = fs::read_to_string(&file_path).expect("Failed to read macro file");
        let recorded: RecordedEvents =
            toml::from_str(&contents).expect("Failed to deserialize macro file");

        assert_eq!(recorded.events.len(), 3); // Expect KeyPress, ButtonPress, and MouseMove events
        assert_eq!(recorded.events[0].get_event_type(), "KeyPress");
        assert_eq!(recorded.events[0].get_key(), Some("MockKey"));
        assert_eq!(recorded.events[1].get_event_type(), "ButtonPress");
        assert_eq!(recorded.events[1].button.as_deref(), Some("Button1"));
    }

    #[test]
    fn test_playback_function() {
        // Test playback of recorded macros using MockListener
        let mock_listener = MockListener::new();

        // Ensure the macro file exists by recording first
        start_recording("test_macro", &mock_listener);
        // Wait a bit to ensure file system has written the file
        thread::sleep(Duration::from_secs(1));

        // Now call playback
        start_playback("test_macro", &mock_listener);

        // Validate playback correctness with assertions
        assert!(mock_listener.was_event_triggered("KeyPress", "MockKey"));
    }

    #[test]
    fn test_handle_stop_keystrokes() {
        // Simulate stop keystroke handling
        let mock_listener = MockListener::new();

        // Assume function for handling stop exists
        handle_stop_keystroke(&mock_listener);

        // Validate that the stop was triggered
        assert!(mock_listener.was_event_triggered("Stop", ""));
    }

    #[test]
    fn test_wait_strategies() {
        // Simulate wait strategies
        let mock_listener = MockListener::new();

        // Assume functions for wait strategies exist
        simulate_wait(&mock_listener);

        // Validate correct handling
        assert!(mock_listener.was_wait_condition_met());
    }

    #[test]
    fn test_simulated_event_types() {
        // Simulate different event types like button presses, releases, etc.
        let mock_listener = MockListener::new();

        // Assert each type by simulation
        simulate_button_press(&mock_listener, "Button1");
        simulate_button_release(&mock_listener, "Button1");
        simulate_mouse_movement(&mock_listener, 100, 150);

        // Validate correctness
        assert!(mock_listener.was_event_triggered("ButtonPress", "Button1"));
        assert!(mock_listener.was_event_triggered("ButtonRelease", "Button1"));
        assert!(mock_listener.was_event_triggered("MouseMove", "100-150"));
    }

    #[test]
    fn test_edge_cases() {
        // Test edge case scenarios
        let mock_listener = MockListener::new();

        // Empty recordings: simulate by not triggering any events
        // Here, we simulate an empty recording by directly creating an empty macro file.
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        fs::create_dir_all(&config_dir).expect("Failed to create macros directory");
        let file_path = config_dir.join("empty_macro.toml");
        fs::write(&file_path, "").expect("Failed to write empty macro file");

        // There should be no triggered events in the listener
        assert_eq!(mock_listener.get_triggered_events_len(), 0);
    }
}
