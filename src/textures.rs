use crate::prelude::*;
use crate::random;
use crate::debug;

use sdl2::render::{TextureCreator, BlendMode};
use sdl2::render::Texture as SDLTexture;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

macro_rules! setter {
    ($name:ident, $type:ty) => {
        pub const fn $name(mut self, value: $type) -> Self {
            self.$name = value;
            self
        }
    };
}

enum Direction {
    Twelve,
    Three,
    Six,
    Nine,
}

impl Direction {
    fn degrees(&self) -> f64 {
        match self {
            Self::Twelve => 0.0,
            Self::Three  => 90.0,
            Self::Six    => 180.0,
            Self::Nine   => 270.0,
        }
    }
}

pub struct TextureTransform{
    direction: Direction,
    scale: f32,
}

impl TextureTransform {
    pub fn new() -> Self {
        Self{
            direction: Direction::Twelve,
            scale: 1.0,
        }
    }

    pub fn random_direction(mut self) -> Self {
        self.direction =
            match random::int(0..=3) {
                0 => Direction::Twelve,
                1 => Direction::Three,
                2 => Direction::Six,
                _ => Direction::Nine,
            };

        self
    }

    // pub fn set_scale(mut self, scale: f32) -> Self {
    //     self.scale = scale;
    //     self
    // }

    setter!(scale, f32);
}

pub type Textures<'a> = HashMap<String, SDLTexture<'a>>;

pub fn load_textures
        <'a>(
            texture_creator: &'a TextureCreator<WindowContext>,
            textures: &mut Textures<'a>
        ) -> Result<()> {

    let timer = debug::Timer::new("loading textures");

    let file = File::open("assets/default/textures/textures.json")?;
    let reader = BufReader::new(file);
    let hashmap: HashMap<String, Vec<Vec<(u8, u8, u8, u8)>>>
        = serde_json::from_reader(reader)?;

    for (id, texture_arr) in hashmap {
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA32,
                texture_arr.len() as u32,
                texture_arr[0].len() as u32)?;
            // .map_err(|e| e.to_string());

        texture.set_blend_mode(BlendMode::Blend);
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..texture_arr.len() {
                for x in 0..texture_arr[0].len() {
                    let offset = y * pitch + x * 4;
                    let color = texture_arr[y][x];

                    buffer[offset + 0] = color.0;
                    buffer[offset + 1] = color.1;
                    buffer[offset + 2] = color.2;
                    buffer[offset + 3] = color.3;
                }
            }
        }).map_err(conv_err!())?;

        textures.insert(id, texture);
    }

    timer.done();

    Ok(())
}

pub fn copy_texture(
        canvas: &mut Canvas,
        textures: &Textures,
        id: &'static str,
        transform: Option<&TextureTransform>,
        rect: rect::Rect) -> Result<()> {

    let texture = 
        &textures.get(id)
        .unwrap_or_else(|| {
            textures.get("error").expect("Could not get texture 'error'")
        });

    let (scale, degrees) = if let Some(transform) = transform {
        (
            transform.scale,
            transform.direction.degrees(),
        )
    } else {
        (
            1.0,
            0.0,
        )
    };
    
    let rect = rect::Rect::from_center(
        rect.center(),
        (rect.width() as f32 * scale) as u32,
        (rect.height() as f32 * scale) as u32
    );

    canvas.copy_ex(
        &texture,
        None,
        Some(rect),
        degrees,
        None,
        false,
        false,
    ).map_err(conv_err!())?;

    Ok(())
}
