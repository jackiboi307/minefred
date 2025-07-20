extern crate sdl2;
extern crate hecs;

mod game;
mod behavior;
mod components;

use game::Game;
use behavior::UpdateData;

use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

const SCREEN_X: u32 = 800;
const SCREEN_Y: u32 = 600;

impl Game {
    fn run(&mut self) {
        self.init();
        self.run_sdl();
    }

    fn run_sdl(&mut self) {
        // Initialize SDL2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window
        let window = video_subsystem
            .window("Minefred", SCREEN_X, SCREEN_Y)
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
