# Macronizer

Macronizer is a command-line tool for recording and playing back system-wide keyboard and mouse events. This application supports both simulated (mock) and real event recording/playback modes, making it ideal for both testing and actual automation use.

## Features

### 1. Event Recording and Playback

- **Keystroke**: Records and simulates keyboard keystrokes.
- **Mouse Click**: Records and simulates mouse clicks (left or right) at a given position.
- **Wait**: Records wait intervals (in milliseconds) between events.

### 2. Macro Storage

- All recorded macros are stored as TOML files in the directory: `~/.config/macronizer/macros/`.
- Each macro file is named based on the given macro name.

### 3. Settings

- A settings file is stored at: `~/.config/macronizer/settings.toml`.
- **Stop Recording/Playback Keystroke(s)**:
  - A user-defined keystroke combination that, when entered during recording, stops the recording and is ignored in the macro.
  - During playback, entering this keystroke combination immediately stops the macro execution.
- **Wait Strategy**:
  - **Record Actual Waits**: Records the actual time pauses between each event and plays them back.
  - **No Waits**: No pauses are added between events during playback.
  - **Constant Wait**: Uses a predefined constant wait time uniformly after each event.

### 4. CLI Commands

- **Record a Macro**

  Use the following command to record a macro:

  ```bash
  mz record <name>
  ```

  This command initiates a 3-second countdown before starting to record keyboard and mouse events.

- **Run a Macro**

  Play back a recorded macro with:

  ```bash
  mz run <name> [number]
  ```

  - `<name>`: Name of the macro to run.
  - `[number]`: (Optional) Number of times to repeat the macro in a row. Defaults to 1 if not provided.

- **Real vs. Simulated Events**

  Use the `--real` flag with the record or run commands to enable real keyboard and mouse event capturing and simulation (using the `rdev` crate). Without this flag, the program uses a simulated (mock) event listener for testing purposes.

  Example:

  ```bash
  mz record mymacro --real
  mz run mymacro 3 --real
  ```

### 5. Libraries Used

- **clap**: For command-line argument parsing. [Documentation](https://docs.rs/clap/latest/clap/).
- **rdev**: For capturing and simulating keyboard and mouse events. [Documentation](https://docs.rs/rdev/latest/rdev/).
- **toml** with **serde**: For parsing and serializing TOML configuration files. [Documentation](https://docs.rs/toml/latest/toml/).

## Code Structure

- **src/macronizer.rs**:
  - Contains the core functionality for event recording and playback.
  - Implements two event listener traits:
    - `MockListener`: For testing and simulation purposes.
    - `RdevListener`: For real event capturing and simulation using `rdev`.
  - Handles conversion between system events and internal representations (`RecordedEvent`).

- **src/main.rs**:
  - CLI entrypoint for the application using `clap`.
  - Processes commands (`record` and `run`) and selects event listener based on the `--real` flag.

- **tests/macronizer_tests.rs**:
  - Contains tests ensuring that recording and playback functionality work as expected using `MockListener`.
  
## Usage

1. **Installation**: Ensure you have Rust installed and clone the repository. Build the project using:

   ```bash
   cargo build --release
   ```

2. **Setup Configuration**:

   Create the settings file (if it does not exist) at `~/.config/macronizer/settings.toml` and define your settings. Example content:

   ```toml
   [settings]
   stop_keystrokes = ["Ctrl", "Alt", "S"]
   wait_strategy = "constant"  # Options: actual, none, constant
   constant_wait_ms = 100
   ```

3. **Recording a Macro**:

   Start recording by specifying a macro name. Example:

   ```bash
   mz record mymacro
   ```

   For real event recording:

   ```bash
   mz record mymacro --real
   ```

4. **Playing Back a Macro**:

   To run a recorded macro once:

   ```bash
   mz run mymacro
   ```

   To run it three times consecutively:

   ```bash
   mz run mymacro 3
   ```

   For real event simulation:

   ```bash
   mz run mymacro --real
   ```

## Future Enhancements

- Refine the conversion logic between `RecordedEvent` and real system events to support a broader range of keystrokes and mouse events.
- Add more robust error handling and logging for troubleshooting during recording and playback.
- Expand configuration options and allow dynamic reloading of settings.

## Conclusion

Macronizer offers a flexible tool for automating repetitive tasks with keyboard and mouse macros. It supports both simulated and real event processing, providing a solid foundation for further customization and enhancements.

For issues or contributions, please refer to the project's repository on GitHub.
