#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_record_macro() {
        // Setup a mock event list
        let mock_events = Arc::new(Mutex::new(Vec::new()));

        // Call record_macro with mock data
        record_macro("test_macro", &mock_events);

        // Verify the event recording logic
        let events = mock_events.lock().unwrap();
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

        // Verify the playback logic
        run_macro("test_macro", 1);

        // No assertions here for now, since it's mainly to ensure the simulate logic runs without error
    }
}
