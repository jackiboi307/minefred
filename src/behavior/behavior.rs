use crate::behavior::base::*;

use sdl2::event::Event;
use sdl2::keyboard::{
    KeyboardState,
    Scancode,
    PressedScancodeIterator,
};

pub struct GameObjectBehavior {
    pub init: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId),

    pub update: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _update_data: &UpdateData),

    pub render: fn(
        &ECSWorld,
        ECSEntityId,
        &mut Canvas
    ),
}

#[allow(dead_code)]
pub struct UpdateData<'a> {
    pub events: Vec<Event>,
    pub keys: KeyboardState<'a>,
}

#[allow(dead_code)]
impl UpdateData<'_> {
    pub fn is_pressed<const S: usize>(&self, scancodes: [Scancode; S]) -> bool {
        for scancode in scancodes {
            if self.keys.is_scancode_pressed(scancode) {
                return true
            }
        }

        return false
    }

    pub fn pressed_keys(&self) -> PressedScancodeIterator<'_> {
        self.keys.pressed_scancodes()
    }
}
