#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_record_event() {
        // MockListener instantiation simulating event handling
        let mock_listener = MockListener;

        // Call the recording function passing the mock listener
        start_recording("test_macro", &mock_listener);

        // Validate that the recordings are saved
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        let file_path = config_dir.join("test_macro.toml");

        // Read and assert the contents of the file
        let contents = fs::read_to_string(file_path).expect("Failed to read macro file");
        let events: Vec<RecordedEvent> = toml::from_str(&contents).expect("Failed to deserialize macro file");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "KeyPress");
        assert_eq!(events[0].key.as_deref(), Some("MockKey"));
    }

    #[test]
    fn test_playback_function() {
        // Test playback of recorded macros using MockListener
        let mock_listener = MockListener;

        // Assume start_playback is a function that plays back macros
        start_playback("test_macro", &mock_listener);

        // Validate playback correctness with assertions
        // Example asserts:
        assert!(mock_listener.was_event_triggered("KeyPress", "MockKey"));
    }

    #[test]
    fn test_handle_stop_keystrokes() {
        // Simulate stop keystroke handling
        let mock_listener = MockListener;

        // Assume function for handling stop exists
        handle_stop_keystroke(&mock_listener);

        // Validate that the stop was triggered
        assert!(mock_listener.was_event_triggered("Stop", ""));
    }

    #[test]
    fn test_wait_strategies() {
        // Simulate wait strategies
        let mock_listener = MockListener;

        // Assume functions for wait strategies exist
        simulate_wait(&mock_listener);

        // Validate correct handling
        assert!(mock_listener.was_wait_condition_met());
    }

    #[test]
    fn test_simulated_event_types() {
        // Simulate different event types like button presses, releases, etc.
        let mock_listener = MockListener;

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
        let mock_listener = MockListener;

        // Empty recordings
        start_recording("empty_macro", &mock_listener);
        assert_eq!(mock_listener.events(), Vec::new());

        // Invalid macro names
        let result = start_recording("invalid?name", &mock_listener);
        assert!(result.is_err());
    }
}
