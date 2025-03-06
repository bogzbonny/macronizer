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
    
    // Optional: Add more tests to cover macro playback and additional scenarios
}
