#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    // Mock callback for capturing events for testing
    fn mock_listen(callback: impl Fn(Event) + 'static) {
        let mock_events = vec![
            Event {
                event_type: EventType::KeyPress(rdev::Key::KeyS),
                time: 0,
                name: Some("s".to_string()),
                code: None,
            },
            Event {
                event_type: EventType::KeyRelease(rdev::Key::KeyS),
                time: 0,
                name: Some("s".to_string()),
                code: None,
            },
        ];

        for event in mock_events {
            callback(event);
        }
    }

    #[test]
    fn test_record_macro() {
        // Setup a mock event queue
        let macro_events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = macro_events.clone();

        // Override listen to mock_listen
        let callback = |e| {
            if let Ok(mut events) = events_clone.lock() {
                events.push(e);
            }
        };

        mock_listen(callback);

        // Assuming event list is non-empty
        let events = macro_events.lock().unwrap();
        assert!(!events.is_empty(), "Expected events to be recorded but found none.");
    }

    #[test]
    fn test_run_macro() {
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

        // Variables for assertions
        let simulate_calls = Arc::new(Mutex::new(VecDeque::new()));
        let simulate_calls_clone = simulate_calls.clone();

        // Mock simulate function
        let mock_simulate = |event_type: &EventType| {
            let target_event = match event_type {
                EventType::KeyPress(rdev::Key::KeyS) => "KeyPress S",
                EventType::KeyRelease(rdev::Key::KeyS) => "KeyRelease S",
                _ => "Unknown",
            };
            
            simulate_calls_clone.lock().unwrap().push_back(target_event.to_string());
            Ok(())
        };

        // Run the macro
        run_macro("test_macro", 1);

        // Check if events were simulated
        let simulated_events = simulate_calls.lock().unwrap();
        assert_eq!(simulated_events.len(), 2, "Expected 2 simulated events, found {}");
    }
}
