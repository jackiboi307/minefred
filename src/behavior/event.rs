use sdl2::keyboard::{Scancode, KeyboardState};
use sdl2::mouse::MouseButton;

use std::collections::HashMap;

enum Button {
    Key(Scancode),

    #[allow(dead_code)]
    Mouse(MouseButton),
}

#[derive(Clone)]
#[allow(dead_code)]
struct Action {
    label: &'static str,
    key: &'static str,
    prolonged: bool,
}

impl Action {
    fn new(label: &'static str, key: &'static str, prolonged: bool) -> Self {
        Self{label, key, prolonged}
    }
}

struct Bind {
    button: Button,
    id: usize,
}

impl Bind {
    fn key(scancode: Scancode, id: usize) -> Self {
        Self{button: Button::Key(scancode), id}
    }
}

#[derive(Clone)]
pub struct Events {
    action_states: Vec<bool>,
    keys: Option<HashMap<&'static str, usize>>,
}

impl Events {
    fn new(size: usize) -> Self {
        Self{
            action_states: vec![false; size],
            keys: None,
        }
    }

    fn add_keys(mut self, keys: HashMap<&'static str, usize>) -> Self {
        self.keys = Some(keys);
        self
    }

    #[allow(dead_code)]
    pub fn id(&self, id: usize) -> bool {
        self.action_states[id]
    }

    pub fn key(&self, key: &'static str) -> bool {
        let keys = self.keys.clone().expect("Event lookup by key is not enabled");
        let Some(id) = keys.get(key) else {
            println!("No action with key '{}' found", key);
            return false
        };
        self.action_states[*id]
    }
}

pub struct EventHandler {
    actions: Vec<Action>,
    binds: Vec<Bind>,
    events: Events,
}

impl EventHandler {
    pub fn reset(&mut self) {
        self.events = Events::new(self.actions.len());
    }

    pub fn register_scancode(&mut self, registered_scancode: Scancode) {
        for bind in &self.binds {
            if !self.actions[bind.id].prolonged {
                match bind.button {
                    Button::Key( scancode ) => {
                        if scancode == registered_scancode {
                            self.events.action_states[bind.id] = true;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn register_keyboard_state(&mut self, keyboard_state: KeyboardState) {
        for bind in &self.binds {
            if self.actions[bind.id].prolonged {
                match bind.button {
                    Button::Key( scancode ) => {
                        if keyboard_state.is_scancode_pressed(scancode) {
                            self.events.action_states[bind.id] = true;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    fn get_keys(&self) -> HashMap<&'static str, usize> {
        let mut keys = HashMap::new();
        for (id, action) in self.actions.clone().into_iter().enumerate() {
            keys.insert(action.key, id);
        }
        keys
    }

    pub fn get_events(&self) -> Events {
        self.events.clone().add_keys(self.get_keys())
    }

    #[allow(dead_code)]
    fn get_id(self, key: &'static str) -> Option<usize> {
        for (id, action) in self.actions.into_iter().enumerate() {
            if key == action.key {
                return Some(id)
            }
        }

        println!("No action with id {} found", key);
        None
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (id, state) in self.events.action_states.clone().into_iter().enumerate() {
            if state {
                println!("{}", self.actions[id].label);
            }
        }
    }

    pub fn new() -> Self {
        let mut actions = Vec::new();
        actions.push(Action::new("Move forward", "MOVE_UP", true));
        actions.push(Action::new("Move back", "MOVE_DOWN", true));
        actions.push(Action::new("Move left", "MOVE_LEFT", true));
        actions.push(Action::new("Move right", "MOVE_RIGHT", true));
        actions.push(Action::new("Run", "RUN", true));

        let num_actions = actions.len();

        let mut binds = Vec::new();
        binds.push(Bind::key(Scancode::W, 0));
        binds.push(Bind::key(Scancode::S, 1));
        binds.push(Bind::key(Scancode::A, 2));
        binds.push(Bind::key(Scancode::D, 3));
        binds.push(Bind::key(Scancode::LShift, 4));

        Self{
            actions,
            binds,
            events: Events::new(num_actions),
        }
    }
}
