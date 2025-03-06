#[cfg(test)]
mod tests {
    use super::*;
    use mockall::automock;
    use std::fs;
    use toml;

    #[automock]
    pub trait EventHandler {
        fn record_event(&self);
        fn playback_event(&self);
    }

    #[test]
    fn test_record_event() {
        let mock_handler = MockEventHandler::new();
        mock_handler.expect_record_event().times(1).return_const(());
        mock_handler.record_event();
    }

    #[test]
    fn test_playback_event() {
        let mock_handler = MockEventHandler::new();
        mock_handler.expect_playback_event().times(1).return_const(());
        mock_handler.playback_event();
    }
}
