#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    // Mock struct for Event as expected by the application
    #[derive(Debug, Clone)]
    struct MockEvent {
        event_type: String,
        details: String,
    }

    // Mock implementation of event listener
    fn mock_listen(callback: impl Fn(MockEvent) + 'static) {
        let mock_events = vec![
            MockEvent {
                event_type: "KeyPress".into(),
                details: "S".into(),
            },
            MockEvent {
                event_type: "KeyRelease".into(),
                details: "S".into(),
            },
        ];

        for event in mock_events {
            callback(event);
        }
    }

    // Mock struct for simulating event playback
    struct MockSimulator {
        simulate_calls: Arc<Mutex<VecDeque<String>>>,
    }

    impl MockSimulator {
        fn new() -> Self {
            Self {
                simulate_calls: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        fn simulate(&self, event_type: &str) {
            let target_event = match event_type {
                "KeyPress" => "KeyPress S",
                "KeyRelease" => "KeyRelease S",
                _ => "Unknown",
            };
            self.simulate_calls.lock().unwrap().push_back(target_event.to_string());
        }
    }

    #[test]
    fn test_record_macro() {
        // Setup a mock event queue
        let macro_events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = macro_events.clone();

        // Override listen to use mock_listen
        let callback = |e: MockEvent| {
            if let Ok(mut events) = events_clone.lock() {
                events.push(MacroEvent {
                    event_type: e.event_type,
                    details: e.details,
                });
            }
        };

        mock_listen(callback);

        // Verify that events have been recorded
        let events = macro_events.lock().unwrap();
        assert!(!events.is_empty(), "Expected events to be recorded but found none.");
    }

    #[test]
    fn test_run_macro() {
        let simulator = MockSimulator::new();

        // Simulate a saved macro in TOML format
        let sample_toml = r#"
            [[event]]
            event_type = "KeyPress"
            details = "S"

            [[event]]
            event_type = "KeyRelease"
            details = "S"
        "#;

        // Save mock macro file
        std::fs::create_dir_all(dirs::config_dir().unwrap().join("macronizer/macros")).unwrap();
        let macro_path = dirs::config_dir().unwrap().join("macronizer/macros/test_macro.toml");
        std::fs::write(&macro_path, sample_toml).unwrap();

        // Mock playback
        for line in sample_toml.lines() {
            if line.contains("KeyPress") {
                simulator.simulate("KeyPress");
            } else if line.contains("KeyRelease") {
                simulator.simulate("KeyRelease");
            }
        }

        // Verify simulated events
        let simulated_events = simulator.simulate_calls.lock().unwrap();
        assert_eq!(simulated_events.len(), 2, "Expected 2 simulated events, recorded {}.");
    }
}
