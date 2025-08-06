use crate::gameobjtype::base::Canvas;
use crate::random;
use crate::types::Error;
use crate::debug;

use sdl2::render::{TextureCreator, BlendMode};
pub use sdl2::render::Texture as SDLTexture;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

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

pub struct TextureComponent{
    id: String,
    direction: Direction,
    // valid: bool,
    scale: f32,
}

impl TextureComponent {
    pub fn new(id: &'static str) -> Self {
        Self{
            id: id.to_string(),
            direction: Direction::Twelve,
            // valid: textures.get(id).is_some(),
            // valid: true,
            scale: 1.0,
        }
    }

    pub fn random_direction(mut self) -> Self {
        // if !self.valid { return self }

        self.direction =
            match random::int(0..=3) {
                0 => Direction::Twelve,
                1 => Direction::Three,
                2 => Direction::Six,
                _ => Direction::Nine,
            };

        self
    }

    pub fn set_scale(mut self, scale: f32) -> Self {
        // if !self.valid { return self }
        self.scale = scale;
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
        })?;

        textures.insert(id, Texture::new(texture));
    }

    timer.done();

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
    
    let rect = rect::Rect::from_center(
        rect.center(),
        (rect.width() as f32 * texture_component.scale) as u32,
        (rect.height() as f32 * texture_component.scale) as u32
    );

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
