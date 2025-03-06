#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use rdev::{Event, EventType};

    // Mock the EventListener trait
    #[automock]
    trait EventListener {
        fn start(&self, callback: impl Fn(Event) + 'static + Send);
    }

    #[test]
    fn test_record_event() {
        // Create a mock event listener
        let mut mock_listener = MockEventListener::new();

        // Set expectations for the mock
        mock_listener.expect_start().returning(|callback| {
            // Simulate event callback
            callback(Event {
                event_type: EventType::KeyPress(rdev::Key::KeyA),
                name: None,
                time: 0,
            });
        });

        // Call the recording function passing the mock listener
        start_recording("test_macro", &mock_listener);

        // Validate that recordings are saved
        let config_dir = dirs::config_dir().unwrap().join("macronizer/macros");
        let file_path = config_dir.join("test_macro.toml");

        // Read and assert the contents of the file
        let contents = fs::read_to_string(file_path).expect("Failed to read macro file");
        let events: Vec<RecordedEvent> = toml::from_str(&contents).expect("Failed to deserialize macro file");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "KeyPress");
        assert_eq!(events[0].key.as_deref(), Some("KeyA"));
    }
    
    // Add more tests to cover the playback and recording behavior as needed
    
}
