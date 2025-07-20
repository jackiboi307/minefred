use sdl2::event::Event;
use sdl2::keyboard::{
    KeyboardState,
    Scancode,
    PressedScancodeIterator,
};

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
