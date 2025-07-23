use crate::constants::{STANDARD_TILE_TEXTURE_SIZE};
use crate::behavior::base::Canvas;
use crate::random;
use crate::types::Error;

use sdl2::render::{TextureCreator, BlendMode};
pub use sdl2::render::Texture as SDLTexture;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

enum Direction {
    TWELVE,
    THREE,
    SIX,
    NINE,
}

impl Direction {
    fn degrees(&self) -> f64 {
        match self {
            Self::TWELVE => 0.0,
            Self::THREE  => 90.0,
            Self::SIX    => 180.0,
            Self::NINE   => 270.0,
        }
    }
}

pub struct TextureComponent{
    id: String,
    direction: Direction,
    valid: bool,
}

impl TextureComponent {
    pub fn new(textures: &Textures, id: &'static str) -> Self {
        Self{
            id: id.to_string(),
            direction: Direction::TWELVE,
            valid: textures.get(id).is_some(),
        }
    }

    pub fn random_direction(mut self) -> Self {
        if !self.valid { return self }

        self.direction =
            match random::int(0..=3) {
                0 => Direction::TWELVE,
                1 => Direction::THREE,
                2 => Direction::SIX,
                _ => Direction::NINE,
            };

        self
    }
}

// TODO remove this struct if it proves to be unnecessary

pub struct Texture<'a> {
    texture: SDLTexture<'a>,
}

impl<'a> Texture<'a> {
    fn new(texture: SDLTexture<'a>) -> Self {
        Self{
            texture,
        }
    }
}

pub type Textures<'a> = HashMap<String, Texture<'a>>;

pub fn load_textures
        <'a>(
            texture_creator: &'a TextureCreator<WindowContext>,
            textures: &mut Textures<'a>
        ) -> Result<(), Error> {

    let file = File::open("assets/default/textures/textures.json")?;
    let reader = BufReader::new(file);
    let hashmap: HashMap<String, Vec<Vec<(u8, u8, u8, u8)>>>
        = serde_json::from_reader(reader)?;

    for (id, texture_arr) in hashmap {
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA32,
                STANDARD_TILE_TEXTURE_SIZE,
                STANDARD_TILE_TEXTURE_SIZE)?;
            // .map_err(|e| e.to_string());
            //

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
        })?;

        textures.insert(id, Texture::new(texture));
    }

    Ok(())
}

pub fn copy_texture(
        canvas: &mut Canvas,
        textures: &Textures,
        texture_component: &TextureComponent,
        rect: rect::Rect) -> Result<(), Error> {

    let texture = 
        &textures.get(&texture_component.id)
        .unwrap_or_else(|| {
            textures.get("error").expect("Could not get texture 'error'")
        });

    canvas.copy_ex(
        &texture.texture,
        None,
        Some(rect),
        texture_component.direction.degrees(),
        None,
        false,
        false,
    )?;

    Ok(())
}
