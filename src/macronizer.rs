use {
    crate::config::{self, Config},
    rdev::EventType,
    std::cell::RefCell,
    std::rc::Rc,
    std::{
        fs,
        sync::{Arc, Mutex},
        {thread, time::Duration},
    },
};

// Container for deserializing events
#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
pub struct RecordedEvents(pub Vec<Event>);

// Starts recording by using the provided event listener
pub fn start_recording(cfg: &Config, name: &str) {
    let macros_dir = config::macros_path();
    fs::create_dir_all(&macros_dir).expect("Failed to create macros directory");

    let file_path = macros_dir.join(format!("{name}.toml"));

    let mut events = Rc::new(RefCell::new(RecordedEvents::default()));

    let mut callback = move |event: rdev::Event| {
        let op_ev = match event.event_type {
            EventType::KeyPress(_) => None,
            EventType::KeyRelease(key) => Some(Event::Keystroke(key)),
            EventType::ButtonPress(button) => Some(Event::MousePress(0, 0, button)),
            EventType::ButtonRelease(button) => Some(Event::MouseRelease(0, 0, button)),
            EventType::MouseMove { x, y } => Some(Event::MouseMove(x as i32, y as i32)),
            EventType::Wheel {
                delta_x: _,
                delta_y: _,
            } => None,
        };
        if let Some(ev) = op_ev {
            events.borrow_mut().0.push(ev);
        }
    };
    if let Err(e) = rdev::listen(move |event| callback(event)) {
        eprintln!("Error in real event listener: {:?}", e);
    }

    let toml_string = toml::to_string_pretty(&events).expect("Failed to serialize recorded events");
    fs::write(file_path, toml_string).expect("Failed to save macro file");
}

// Starts playback by deserializing events and passing them to the provided event listener
pub fn start_playback(_cfg: &Config, name: &str) {
    let macros_dir = config::macros_path();
    let file_path = macros_dir.join(format!("{}.toml", name));

    // get the macro for the name and deserialize it

    let Ok(contents) = fs::read_to_string(file_path) else {
        println!("Macro not found");
        return;
    };

    let evs: RecordedEvents = match toml::from_str(&contents) {
        Ok(evs) => evs,
        Err(e) => {
            println!("Failed to deserialize macro file: {:?}", e);
            return;
        }
    };

    for ev in evs.0 {
        // TODO check for stop
        ev.simulate();
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Event {
    /// keystroke event  
    Keystroke(rdev::Key),
    /// x, y, button
    MousePress(i32, i32, rdev::Button),
    MouseMove(i32, i32),
    MouseRelease(i32, i32, rdev::Button),
    /// wait in milliseconds
    Wait(u64),
}

impl Event {
    pub fn simulate(&self) {
        match self {
            Event::Keystroke(key) => {
                let ev_type = rdev::EventType::KeyPress(*key);
                rdev::simulate(&ev_type).unwrap();
                thread::sleep(std::time::Duration::from_millis(1));
                let ev_type = rdev::EventType::KeyRelease(*key);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MousePress(x, y, button) => {
                let ev_type = rdev::EventType::MouseMove {
                    x: *x as f64,
                    y: *y as f64,
                };
                rdev::simulate(&ev_type).unwrap();
                thread::sleep(std::time::Duration::from_millis(1));
                let ev_type = rdev::EventType::ButtonPress(*button);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MouseMove(x, y) => {
                let ev_type = rdev::EventType::MouseMove {
                    x: *x as f64,
                    y: *y as f64,
                };
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MouseRelease(x, y, button) => {
                let ev_type = rdev::EventType::MouseMove {
                    x: *x as f64,
                    y: *y as f64,
                };
                rdev::simulate(&ev_type).unwrap();
                thread::sleep(std::time::Duration::from_millis(1));
                let ev_type = rdev::EventType::ButtonRelease(*button);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::Wait(ms) => std::thread::sleep(std::time::Duration::from_millis(*ms)),
        }
    }
}
