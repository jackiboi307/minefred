mod ui;
mod game;
mod debug;
mod types;
mod utils;
mod event;
mod error;
mod random;
mod prelude;
mod textures;
mod constants;
mod components;
mod gameobjtype;

use game::Game;
use prelude::*;
use constants::*;

use sdl2::pixels::Color;

use std::time::Duration;

fn run() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("Minefred", SCREEN_X.into(), SCREEN_Y.into());
    let window = window
        .position_centered();

    if RESIZABLE {
        window.resizable();
    }
    
    let window = window.build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut game = Game::new();
    game.init_textures(&texture_creator).context("loading textures")?;
    game.init_fonts(&ttf_context, &texture_creator).context("loading fonts")?;
    game.init().context("initializing")?;

    'main: loop {
        let timer = debug::Timer::new("WHOLE FRAME");

        if game.update(&mut event_pump).context("updating")? {
            break 'main;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        game.render(&mut canvas).context("rendering")?;

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

        std::thread::sleep(Duration::from_millis(
            (1000 / FPS).saturating_sub(elapsed)));
    }

    println!();

    Ok(())
}

fn main() {
    let res = run();

    if let Err(errors) = res {
        // horrible fucking code because anyhow / eyre sucks
        let location = format!("{:?}", errors);
        let location = location
            .split("\n")
            .last()
            .unwrap_or_else(|| "")
            .trim_start();

        println!(
            "fatal error: {}\n    {}\n\ncaused by",
            errors.chain().last().unwrap(),
            location,
        );

        for (i, err) in errors.chain().enumerate() {
            println!("    {}: {}", i+1, err);
        }
    }
}
