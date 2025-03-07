use {
    crate::config::{self, Config, WaitStrategy},
    rdev::EventType,
    std::{cell::RefCell, fs, rc::Rc, thread, time::Instant},
};

// Container for deserializing events
#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
pub struct RecordedEvents {
    events: Vec<Event>,
}

// Starts recording by using the provided event listener
pub fn record(cfg: &Config, name: String) {
    let events = Rc::new(RefCell::new(RecordedEvents::default()));
    let mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
    let mouse_pressed = Rc::new(RefCell::new(false));
    let recent_keys = Rc::new(RefCell::new(Vec::new()));
    let last_event_time = Rc::new(RefCell::new(None::<Instant>));

    // set the recording_initial_wait_ms
    events
        .borrow_mut()
        .events
        .push(Event::Wait(cfg.recording_initial_wait_ms));

    let cfg_ = cfg.clone();
    let events_ = events.clone();
    let recent_keys_ = recent_keys.clone();
    let callback = move |event: rdev::Event| {
        let op_ev = match event.event_type {
            EventType::KeyPress(key) => {
                recent_keys_.borrow_mut().push(key);
                println!("adding event: keypress {:?}", key);
                Some(Event::KeyPress(key))
            }
            EventType::KeyRelease(key) => {
                println!("adding event: keyrelease {:?}", key);
                Some(Event::KeyRelease(key))
            }
            EventType::ButtonPress(button) => {
                println!("adding event: mousepress {:?}", button);
                recent_keys_.replace(Vec::new());
                mouse_pressed.replace(true);
                let m = *mouse_pos.borrow();
                Some(Event::MousePress(MouseEventButton {
                    x: m.0,
                    y: m.1,
                    button,
                }))
            }
            EventType::ButtonRelease(button) => {
                println!("adding event: mouserelease {:?}", button);
                recent_keys_.replace(Vec::new());
                mouse_pressed.replace(false);
                let m = *mouse_pos.borrow();
                Some(Event::MouseRelease(MouseEventButton {
                    x: m.0,
                    y: m.1,
                    button,
                }))
            }
            EventType::MouseMove { x, y } => {
                println!("adding event: mousemove {:?}", (x, y));
                mouse_pos.replace((x, y));
                recent_keys_.replace(Vec::new());
                if cfg_.record_non_drag_mouse_moves || *mouse_pressed.borrow() {
                    Some(Event::MouseMove(MouseEventMove { x, y }))
                } else {
                    None
                }
            }
            EventType::Wheel {
                delta_x: _,
                delta_y: _,
            } => None,
        };
        if let Some(ev) = op_ev {
            match cfg_.wait_strategy {
                WaitStrategy::Actual => {
                    let now = Instant::now();
                    if let Some(last_event_time) = last_event_time.borrow_mut().take() {
                        let ms = now.duration_since(last_event_time).as_millis() as u64;
                        events_.borrow_mut().events.push(Event::Wait(ms));
                        println!("adding event: wait {}", ms);
                    }
                    last_event_time.replace(Some(now));
                }
                WaitStrategy::ConstantMS(ms) => {
                    // TODO more complex constant wait strategy
                    events_.borrow_mut().events.push(Event::Wait(ms));
                }
            };
            events_.borrow_mut().events.push(ev);
        }
        // break if the end keys recent_keys match the stop keys
        if recent_keys_.borrow().ends_with(&cfg_.stop_keystrokes) {
            let mut events = events_.borrow().clone();
            let mut to_pop = cfg_.stop_keystrokes.clone();
            // move through the events in reverse popping everything that matches the stop keys
            for i in (0..events.events.len()).rev() {
                if let Event::KeyPress(key) = &events.events[i] {
                    if to_pop.last() == Some(key) {
                        to_pop.pop();
                        if to_pop.is_empty() {
                            events.events.pop();
                            break;
                        }
                    }
                }
                events.events.pop();
            }

            let mut toml_string =
                toml::to_string(&events).expect("Failed to serialize recorded events");

            // manually make the toml string nicer
            toml_string = toml_string.replace("[[events]]\n\n", "[[events]]\n");

            let macros_dir = config::macros_path();
            fs::create_dir_all(&macros_dir).expect("Failed to create macros directory");
            let file_path = macros_dir.join(format!("{name}.toml"));
            fs::write(file_path, toml_string).expect("Failed to save macro file");

            // exit with a silent panic (as you can't exit the listen loop within rdev currently)
            let _ = std::fs::File::create("/dev/null").map(|_f| -> Result<(), std::io::Error> {
                std::panic::set_hook(Box::new(|_| {})); // Silence panic
                panic!("silent panic");
            });
        }
    };

    if let Err(_e) = rdev::listen(callback.clone()) {
        // ignore the error which will occur at the end of the recording
    }
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

    for ev in evs.events {
        // TODO check for stop
        ev.simulate();
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    KeyPress(rdev::Key),
    KeyRelease(rdev::Key),
    MousePress(MouseEventButton),
    MouseRelease(MouseEventButton),
    MouseMove(MouseEventMove),
    /// wait in milliseconds
    Wait(u64),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MouseEventButton {
    pub x: f64,
    pub y: f64,
    pub button: rdev::Button,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct MouseEventMove {
    pub x: f64,
    pub y: f64,
}

impl Event {
    pub fn simulate(&self) {
        match self {
            Event::KeyPress(key) => {
                let ev_type = rdev::EventType::KeyPress(*key);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::KeyRelease(key) => {
                let ev_type = rdev::EventType::KeyRelease(*key);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MousePress(m) => {
                let MouseEventButton { x, y, button } = m;
                let ev_type = rdev::EventType::MouseMove { x: *x, y: *y };
                rdev::simulate(&ev_type).unwrap();
                thread::sleep(std::time::Duration::from_millis(1));
                let ev_type = rdev::EventType::ButtonPress(*button);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MouseMove(m) => {
                let MouseEventMove { x, y } = m;
                let ev_type = rdev::EventType::MouseMove { x: *x, y: *y };
                rdev::simulate(&ev_type).unwrap();
            }
            Event::MouseRelease(m) => {
                let MouseEventButton { x, y, button } = m;
                let ev_type = rdev::EventType::MouseMove { x: *x, y: *y };
                rdev::simulate(&ev_type).unwrap();
                thread::sleep(std::time::Duration::from_millis(1));
                let ev_type = rdev::EventType::ButtonRelease(*button);
                rdev::simulate(&ev_type).unwrap();
            }
            Event::Wait(ms) => std::thread::sleep(std::time::Duration::from_millis(*ms)),
        }
    }
}
