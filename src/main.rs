extern crate sdl2;
extern crate hecs;
extern crate rand;
extern crate serde_json;

mod game;
mod types;
mod random;
mod behavior;
mod textures;
mod constants;
mod components;

use game::Game;
use behavior::UpdateData;
use constants::{SCREEN_X, SCREEN_Y};

use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::Duration;

fn run() {
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
    let texture_creator = canvas.texture_creator();

    let mut game = Game::new();
    game.init_textures(&texture_creator);
    game.init();

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

        game.update(&update_data);

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        game.render(&mut canvas);

        // Present the canvas
        canvas.present();

        // Wait for a short duration
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn main() {
    run();
}
