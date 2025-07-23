use crate::behavior::base::*;
use crate::types::*;
use crate::textures::Textures;

use sdl2::event::Event;
use sdl2::rect;
use sdl2::keyboard::{
    KeyboardState,
    Scancode,
    PressedScancodeIterator,
};

#[derive(Copy, Clone)]
pub struct GameObjectBehavior {
    pub init: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _textures: &Textures) -> Result<(), Error>,

    pub update: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _update_data: &UpdateData) -> Result<(), Error>,

    pub render: fn(
        _ecs: &ECSWorld,
        _ecs_id: ECSEntityId,
        _render_info: &RenderInfo,
        textures: &Textures,
        _canvas: &mut Canvas) -> Result<(), Error>,
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

#[allow(dead_code)]
pub struct RenderInfo {
    pub screen: Rect,
    pub rect: rect::Rect,
}
