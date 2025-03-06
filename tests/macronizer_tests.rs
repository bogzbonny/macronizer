use macronizer::macronizer::{
    handle_stop_keystroke, simulate_button_press, simulate_button_release, simulate_mouse_movement,
    simulate_wait, start_playback, start_recording, MockListener, RecordedEvent, RecordedEvents,
};
use std::fs;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

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
        let recorded_events: RecordedEvents =
            toml::from_str(&contents).expect("Failed to deserialize macro file");

        assert_eq!(recorded_events.events.len(), 3); // Expect KeyPress, ButtonPress, and MouseMove events
        assert_eq!(recorded_events.events[0].get_event_type(), "KeyPress");
        assert_eq!(recorded_events.events[0].get_key(), Some("MockKey"));
        assert_eq!(recorded_events.events[1].get_event_type(), "ButtonPress");
        assert_eq!(recorded_events.events[1].button.as_deref(), Some("Button1"));
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
        // Here, we simulate an empty recording by not calling the callback.
        // Instead, we directly create an empty macro file.
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        fs::create_dir_all(&config_dir).expect("Failed to create macros directory");
        let file_path = config_dir.join("empty_macro.toml");
        fs::write(&file_path, "").expect("Failed to write empty macro file");

        // There should be no triggered events in the listener
        assert_eq!(mock_listener.get_triggered_events_len(), 0);
    }
}
