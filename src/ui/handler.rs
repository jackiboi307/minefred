use crate::prelude::*;

use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::rect;

use std::collections::HashMap;

pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
}

impl UIRect {
    pub fn centered() -> Self {
        Self {
            x: 0.25,
            y: 0.25,
            width: 0.5,
            height: 0.5,
            visible: false,
        }
    }

    pub fn to_rect(&self, screen_size: (i32, i32)) -> rect::Rect {
        rect::Rect::new(
            (screen_size.0 as f32 * self.x) as i32,
            (screen_size.1 as f32 * self.y) as i32,
            (screen_size.0 as f32 * self.width) as u32,
            (screen_size.1 as f32 * self.height) as u32,
        )
    }

    pub fn update(&mut self,
            event_pump: &mut EventPump,
            screen_size: (i32, i32)) -> bool {

        // let keyboard = event_pump.keyboard_state();
        let mouse = event_pump.mouse_state();

        // if keyboard.is_scancode_pressed(Scancode::LCtrl) &&
            // if mouse.left() {
            //     self.x += mouse_delta.0 as f32 / screen_size.0 as f32;
            //     self.y += mouse_delta.1 as f32 / screen_size.1 as f32;
            // } else if mouse.right() {
            //     self.width  += mouse_delta.0 as f32 / screen_size.0 as f32;
            //     self.height += mouse_delta.1 as f32 / screen_size.1 as f32;
            // }

        // NOTE deleted code allows rect to be moved

        self.to_rect(screen_size).contains_point((mouse.x(), mouse.y()))
    }
}

pub struct UIHandler {
    pub rects: HashMap<&'static str, UIRect>,
}

impl UIHandler {
    pub fn new() -> Self {
        Self {
            rects: HashMap::new(),
        }
    }

    pub fn get(&self, key: &'static str) -> Option<&UIRect> {
        self.rects.get(key)
    }

    pub fn get_mut(&mut self, key: &'static str) -> Option<&mut UIRect> {
        self.rects.get_mut(key)
    }

    pub fn add(&mut self, key: &'static str) {
        self.rects.insert(key, UIRect::centered());
    }

    pub fn update(&mut self,
            event_pump: &mut EventPump,
            screen_size: (i32, i32)) -> bool {

        let mut ui_hovered = false;

        for (_, rect) in &mut self.rects {
            if rect.visible {
                ui_hovered = ui_hovered || rect.update(event_pump, screen_size);
            }
        }

        ui_hovered
    }
}
