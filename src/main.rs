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
use types::Error;

use sdl2::event::Event;
use sdl2::pixels::Color;

use std::time::{Duration, Instant};

const FPS: u64 = 60;

fn run() -> Result<(), Error> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create a window
    let window = video_subsystem
        .window("Minefred", SCREEN_X.into(), SCREEN_Y.into())
        .position_centered()
        .build()?;

    // Create a canvas
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let mut game = Game::new();
    game.init_textures(&texture_creator)?;
    game.init();

    // Main loop
    'main: loop {
        let start_time = Instant::now();

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

        game.update(&update_data)?;

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        game.render(&mut canvas)?;

        // Present the canvas
        canvas.present();

        let elapsed_time = start_time.elapsed().as_millis() as u64;

        // Wait for a short duration
        std::thread::sleep(Duration::from_millis(
            (1000 / FPS).saturating_sub(elapsed_time)));
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
