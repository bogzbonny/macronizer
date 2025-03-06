pub mod macronizer {
    pub use crate::{
        handle_stop_keystroke, simulate_button_press, simulate_button_release,
        simulate_mouse_movement, simulate_wait, start_recording, MockListener, RecordedEvent,
    };

    // If `start_playback` is a part of this too, ensure it is exposed similarly
}
