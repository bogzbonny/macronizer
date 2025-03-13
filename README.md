# MACRONIZER

A cross-platform recording and playback system for keyboard and mouse macros.
All recorded macros are stored as `.toml`  and can be manually edited.

## Installation
currently unreleased - so pull code and `cargo install --path .`

also see: https://github.com/Narsil/rdev?tab=readme-ov-file#os-caveats

## Usage
**Recording a Macro**:
   Start recording by specifying a macro name. Example:
```bash
macronizer rec mymacro
... do some stuff
hit ESC ESC ESC to exit
```

NOTE due to limitations of rdev this will exiting will generate a silent panic
(so it may look like an error but it's not).

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

- **Stop Recording/Playback Keystroke(s)**:
  - A user-defined keystroke combination that, when entered during recording,
    stops the recording and is ignored in the macro.
  - The default stop sequence is \<Esc\>\<Esc\>\<Esc\>
- **Wait Strategy**:
  - **Record Actual Waits**: Records the actual time pauses between each event
    and plays them back.
  - **Constant Wait**: Uses a predefined constant wait time uniformly after each
    event.

## Files
- macros are automatically stored in: `~/.config/macronizer/macros/<macro-name>.toml`
- The settings file is: `~/.config/macronizer/settings.toml`.

## Alternatives
 - keyboard maestro (mac) 
 - autohotkey (windows) 
 - probably a few others lol

## TODO
(open to contributions) 
- inline mini TUI manager to run, edit, rename, delete, macros
- change the repo such that the binary will be named 'mz'
- default usage should be "run" so the user can just type: 
    `macronizer my` and that will be the same as `macronizer run mymacro` SO
    long as there is only one macro that begins with `my` otherwise macronizer
    should error stating the two macros which have conflicting names
- Usage of positional parameters, for instance mouse positions should be able to
  be "50%" of the screen
- Ability to pass in custom arguments which are used for specific wait times for
  instance, or screen positions for that matter
- settings command to open the settings file with $EDITOR
- edit command to open the macro in $EDITOR
- overwrite configuration options with flags
- run in server mode for keyboard macros without having to open macronizer?
  - see grab: https://docs.rs/rdev/latest/rdev/#grabbing-global-events-requires-unstable_grab-feature
  - could map some commands to stuff like `ctrl+ctrl+ctrl` which have no normal
    effect
  - presumably there should be a "default" grouping of server commands which are
    initiated when the user types `macronizer serve` - however there could maybe
    be multiple server groups stored in `~/.config/macronizer/servers/`
- map midi events to macros using https://github.com/Boddlnagg/midir
  - should be able to map a "midi stroke if another specified midi stroke
    happened less than 1 second previously"

- Post to r/audioengineering r/rust
