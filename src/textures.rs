use crate::constants::{STANDARD_TILE_TEXTURE_SIZE};
use crate::behavior::base::Canvas;

use sdl2::render::TextureCreator;
pub use sdl2::render::Texture as SDLTexture;
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Eq, Hash, PartialEq)]
pub struct TextureId(pub String);

impl TextureId {
    pub fn new(string: &'static str) -> Self {
        Self(string.to_string())
    }
}

struct TextureProperties {
    random_rotate: bool,
}

pub struct Texture<'a> {
    texture: SDLTexture<'a>,
    properties: TextureProperties,
}

impl<'a> Texture<'a> {
    fn new(texture: SDLTexture<'a>) -> Self {
        Self{
            texture,
            properties: TextureProperties{
                random_rotate: true,
            },
        }
    }
}

pub type Textures<'a> = HashMap<TextureId, Texture<'a>>;

pub fn load_textures
        <'a>(
            texture_creator: &'a TextureCreator<WindowContext>,
            textures: &mut Textures<'a>
        ) {

    let file = File::open("assets/default/textures/textures.json").unwrap();
    let reader = BufReader::new(file);
    let hashmap: HashMap<String, Vec<Vec<(u8, u8, u8, u8)>>>
        = serde_json::from_reader(reader).unwrap();

    for (id, texture_arr) in hashmap {
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA32,
                STANDARD_TILE_TEXTURE_SIZE,
                STANDARD_TILE_TEXTURE_SIZE)
            .map_err(|e| e.to_string()).unwrap();
        
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
        }).unwrap();

        textures.insert(TextureId(id), Texture::new(texture));
    }
}

pub fn copy_texture(canvas: &mut Canvas, texture: &Texture, rect: rect::Rect) {
    canvas.copy(
        &texture.texture,
        None,
        Some(rect),
    ).unwrap();
}
