use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

use std::collections::{HashMap, HashSet};

pub type ActionIdType = u8;

#[derive(Eq, Hash, PartialEq)]
pub enum Button {
    Key(Scancode),
    Mouse(MouseButton),
}

impl Into<Button> for Scancode {
    fn into(self) -> Button { Button::Key(self) }}

impl Into<Button> for MouseButton {
    fn into(self) -> Button { Button::Mouse(self) }}

#[derive(Clone)]
struct Action {
    #[allow(dead_code)]
    label: &'static str,

    key: &'static str,
    prolonged: bool,

    #[allow(dead_code)]
    local: bool,
}

#[derive(Clone)]
pub struct ActionState {
    momentary: HashSet<&'static str>,
    prolonged: HashSet<&'static str>,
}

impl ActionState {
    pub fn new() -> Self {
        Self {
            momentary: HashSet::new(),
            prolonged: HashSet::new(),
        }
    }

    pub fn key(&self, key: &'static str) -> bool {
        self.prolonged.contains(key) || self.momentary.contains(key)
    }

    pub fn update(&mut self, handler: &ActionHandler, updates: &ActionUpdates) {
        self.momentary.clear();

        for id in &updates.add {
            let action = &handler.actions[*id as usize];
            if action.prolonged {
                self.prolonged.insert(action.key);
            } else {
                self.momentary.insert(action.key);
            }
        }

        for id in &updates.remove {
            self.prolonged.take(handler.actions[*id as usize].key);
        }
    }
}

pub struct ActionUpdates {
    add: Vec<ActionIdType>,
    remove: Vec<ActionIdType>,
}

impl ActionUpdates {
    pub fn new() -> Self {
        Self {
            add: Vec::new(),
            remove: Vec::new(),
        }
    }

    pub fn register_event(
            &mut self,
            handler: &ActionHandler,
            event: impl Into<Button>,
            down: bool) {

        let id =
            if let Some(id) = handler.binds.get(&event.into()) {
                id
            } else {
                return
            };

        if down {
            self.add.push(*id);
        } else {
            if handler.actions[*id as usize].prolonged {
                self.remove.push(*id);
            }
        }
    }

    pub fn clear(&mut self) {
        self.add.clear();
        self.remove.clear();
    }
}

pub struct ActionHandler {
    actions: Box<[Action]>,
    binds: HashMap<Button, ActionIdType>,
    keys: HashMap<&'static str, ActionIdType>,
}

impl ActionHandler {
    pub fn new() -> Self {
        let mut s = Self {
            actions: Box::new([]),
            binds: HashMap::new(),
            keys: HashMap::new(),
        };

        s.init();

        s
    }

    pub fn init(&mut self) {
        let mut actions = Vec::new();

        for (button, action) in DEFAULT_BINDS {
            actions.push(action.clone());
            let id: ActionIdType = (actions.len() - 1)
                .try_into().expect("couldn't add action. action id type is too small");
            self.binds.insert(button, id);
            self.keys.insert(action.key, id);
        }

        self.actions = actions.into_boxed_slice();
    }
}

const DEFAULT_BINDS: [(Button, Action); 7] = [
    (Button::Key(Scancode::W), Action {
        label: "Move up",
        key: "move_up",
        prolonged: true,
        local: false, }),
    (Button::Key(Scancode::S), Action {
        label: "Move down",
        key: "move_down",
        prolonged: true,
        local: false, }),
    (Button::Key(Scancode::A), Action {
        label: "Move left",
        key: "move_left",
        prolonged: true,
        local: false, }),
    (Button::Key(Scancode::D), Action {
        label: "Move right",
        key: "move_right",
        prolonged: true,
        local: false, }),
    (Button::Key(Scancode::LShift), Action {
        label: "Run",
        key: "run",
        prolonged: true,
        local: false, }),
    (Button::Mouse(MouseButton::Right), Action {
        label: "Attack / Break",
        key: "attack",
        prolonged: false,
        local: false, }),
    (Button::Key(Scancode::E), Action {
        label: "Toggle inventory",
        key: "toggle_inventory",
        prolonged: false,
        local: true, }),
];
