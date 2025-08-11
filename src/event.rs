use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

use std::collections::{HashMap, HashSet};

type ActionIdType = u16;

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

pub struct EventHandler {
    actions: Box<[Action]>,
    binds: HashMap<Button, ActionIdType>,
    keys: HashMap<&'static str, ActionIdType>,
    active: HashSet<ActionIdType>,
}

impl EventHandler {
    pub fn new() -> Self {
        let mut s = Self {
            actions: Box::new([]),
            binds: HashMap::new(),
            keys: HashMap::new(),
            active: HashSet::new(),
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

    pub fn reset(&mut self) {
        self.active.retain(|&id| self.actions[id as usize].prolonged);
    }

    pub fn register_event(&mut self, event: impl Into<Button>, down: bool) {
        let id =
            if let Some(id) = self.binds.get(&event.into()) {
                id
            } else {
                return
            };

        if down {
            self.active.insert(*id);
        } else {
            if self.actions[*id as usize].prolonged {
                self.active.remove(id);
            }
        }
    }

    pub fn key(&self, key: &'static str) -> bool {
        let id =
            if let Some(id) = self.keys.get(key) {
                id
            } else {
                println!("invalid action key: '{}'", key);
                return false
            };

        self.active.contains(id)
    }
}

const DEFAULT_BINDS: [(Button, Action); 6] = [
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
];
