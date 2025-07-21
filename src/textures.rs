use crate::random;
use crate::constants::{MIN, MAX, STANDARD_TILE_TEXTURE_SIZE};

use sdl2::render::TextureCreator;
pub use sdl2::render::Texture;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;

use std::collections::HashMap;

type Color = (u8, u8, u8);

const RED:   Color = (255, 0, 0);
const GREEN: Color = (0, 255, 0);
const BLUE:  Color = (0, 0, 255);

#[derive(Eq, Hash, PartialEq)]
pub struct TextureId(pub &'static str);

pub type Textures<'a> = HashMap<TextureId, Texture<'a>>;

pub fn gen_texture
        <'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            STANDARD_TILE_TEXTURE_SIZE,
            STANDARD_TILE_TEXTURE_SIZE)
        .map_err(|e| e.to_string()).unwrap();
    
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..STANDARD_TILE_TEXTURE_SIZE as usize {
            for x in 0..STANDARD_TILE_TEXTURE_SIZE as usize {
                let offset = y * pitch + x * 3;

                let color = match random::int(0..=2) {
                    MIN..=0 => RED,
                    1       => GREEN,
                    2..=MAX => BLUE,
                };

                buffer[offset] =     color.0;
                buffer[offset + 1] = color.1;
                buffer[offset + 2] = color.2;
            }
        }
    }).unwrap();

    texture
}
