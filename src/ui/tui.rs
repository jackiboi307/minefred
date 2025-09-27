use crate::prelude::*;

// use sdl2::pixels::Color;
// use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::{
    TextureCreator,
    Texture,
};

use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;

const GLYPHS: &'static str = "\
abcdefghijklmnopqrstuvwxyz\
ABCDEFGHIJKLMNOPQRSTUVWXYZ\
0123456789\
!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \
åäö";

pub struct RenderedFont<'a> {
    glyph_textures: HashMap<&'a str, Texture<'a>>,
    char_size: (u32, u32),
}

impl<'a> RenderedFont<'a> {
    pub fn empty() -> Self {
        Self {
            glyph_textures: HashMap::new(),
            char_size: (0, 0),
        }
    }

    pub fn new(
            ttf_context: &'a Sdl2TtfContext,
            texture_creator: &'a TextureCreator<WindowContext>,
            path: &'static str) -> Result<Self> {
        
        let mut glyph_textures = HashMap::new();

        let mut font = ttf_context
            .load_font(path, 15)
            .map_err(conv_err!())?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let char_size = font.size_of_char('m').unwrap();

        for glyph in GLYPHS.graphemes(true) {
            let surface = font
                .render_char(glyph.chars().next().unwrap())
                .blended((255, 255, 255))
                .map_err(conv_err!())?;

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(conv_err!())?;

            glyph_textures.insert(glyph, texture);
        }

        Ok(Self {
            glyph_textures,
            char_size,
        })
    }

    pub fn pixels_to_chars_x(&self, pixels: u32) -> u32 {
        pixels / self.char_size.0
    }

    pub fn pixels_to_chars_y(&self, pixels: u32) -> u32 {
        pixels / self.char_size.1
    }

    pub fn chars_to_pixels_x(&self, chars: u32) -> u32 {
        chars * self.char_size.0
    }

    pub fn chars_to_pixels_y(&self, chars: u32) -> u32 {
        chars * self.char_size.1
    }

    pub fn render_text(
            &mut self,
            canvas: &mut Canvas,
            pos: (u32, u32),
            size: (u32, u32),
            text: String,
            fg: (u8, u8, u8),
            bg: Option<(u8, u8, u8)>) -> Result<()> {

        let mut x = 0;

        for glyph in text.graphemes(true) {
            let texture = self.glyph_textures.get_mut(glyph);

            if let Some(mut texture) = texture {
                if (x as u32) < size.0 {
                    let rect = Rect::new(
                        (pos.0 as i32 + x) * self.char_size.0 as i32,
                        (pos.1 * self.char_size.1) as i32,
                        self.char_size.0,
                        self.char_size.1
                    );

                    if let Some(bg) = bg {
                        canvas.set_draw_color((bg.0, bg.1, bg.2));
                        canvas.fill_rect(rect)
                            .map_err(conv_err!())?;
                    }

                    texture.set_color_mod(fg.0, fg.1, fg.2);
                    canvas.copy(&texture, None, rect).map_err(conv_err!())?;
                }
            }

            x += 1;
        }

        Ok(())
    }
}
