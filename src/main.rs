extern crate sdl2;
extern crate hecs;
extern crate rand;

mod game;
mod types;
mod random;
mod behavior;
mod components;

use game::Game;
use behavior::UpdateData;
use types::*;

use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

// Constants

pub const SCREEN_X: SizeType = 800;
pub const SCREEN_Y: SizeType = 600;

pub const TILE_SIZE: Rect = Rect{width: 50, height: 50};

pub const MIN: i32 = i32::MIN;
pub const MAX: i32 = i32::MAX;

impl Game {
    fn run(&mut self) {
        self.run_sdl();
    }

    fn run_sdl(&mut self) {
        // Initialize SDL2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window
        let window = video_subsystem
            .window("Minefred", SCREEN_X.into(), SCREEN_Y.into())
            .position_centered()
            .build()
            .unwrap();

        // Create a canvas
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        // Main loop
        'main: loop {
            let mut events = Vec::<Event>::new();

            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'main;
                    },

                    _ => {
                        events.push(event);
                    }
                }
            }

            let update_data = UpdateData{
                events,
                keys: event_pump.keyboard_state(),
            };

            self.update(&update_data);

            // Clear the canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            self.render(&mut canvas);

            // Present the canvas
            canvas.present();

            // Wait for a short duration
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
