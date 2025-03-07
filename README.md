# MACRONIZER

Record and playback system keyboard and mouse macros. All recorded macros are
stored as `.toml` files in the directory: `~/.config/macronizer/macros/` and can be
manually edited.

## Installation
```bash
cargo install macronizer
```

also see: https://github.com/Narsil/rdev?tab=readme-ov-file#os-caveats

## Usage
**Recording a Macro**:
   Start recording by specifying a macro name. Example:
```bash
macronizer rec mymacro
```

**Playing Back a Macro**:
To run a recorded macro once:
```bash
macronizer run mymacro
```

To run it three times consecutively:
```bash
macronizer run mymacro 3
```

## Settings
- The settings file is: `~/.config/macronizer/settings.toml`.
- **Stop Recording/Playback Keystroke(s)**:
  - A user-defined keystroke combination that, when entered during recording,
    stops the recording and is ignored in the macro.
  - The default stop sequence is <Esc><Esc><Esc>
- **Wait Strategy**:
  - **Record Actual Waits**: Records the actual time pauses between each event
    and plays them back.
  - **Constant Wait**: Uses a predefined constant wait time uniformly after each
    event.

