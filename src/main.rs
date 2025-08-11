extern crate sdl2;
extern crate hecs;
extern crate rand;
extern crate serde_json;

mod game;
mod debug;
mod types;
mod utils;
mod event;
mod random;
mod textures;
mod constants;
mod components;
mod gameobjtype;

use game::Game;
use constants::{SCREEN_X, SCREEN_Y};
use types::Error;

use sdl2::pixels::Color;

use std::time::Duration;
use std::backtrace::Backtrace;

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

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let mut game = Game::new();
    game.init_textures(&texture_creator)?;
    game.init()?;

    // Main loop
    'main: loop {
        let timer = debug::Timer::new("WHOLE FRAME");

        if game.update(&mut event_pump)? {
            break 'main;
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        game.render(&mut canvas)?;

        // Present the canvas
        canvas.present();

        let elapsed = timer.elapsed() as u64;
        if debug::PRINT_LAG && elapsed > 1000 / FPS {
            println!(
                "lag! +{:<2} ms ({} ms) ({} fps)",
                elapsed - 1000 / FPS,
                elapsed,
                1000 / elapsed
            );
        }

        // Wait for a short duration
        std::thread::sleep(Duration::from_millis(
            (1000 / FPS).saturating_sub(elapsed)));
    }

    println!();

    Ok(())
}

fn main() {
    let result = run();
    if let Err(err) = result {
        print!("Fatal error!\n{}\nBacktrace:\n{}",
            err,
            Backtrace::force_capture(),
        );
    }
}
