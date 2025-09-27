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
    char_textures: HashMap<&'a str, Texture<'a>>,
    char_size: (u32, u32),
}

impl<'a> RenderedFont<'a> {
    pub fn empty() -> Self {
        Self {
            char_textures: HashMap::new(),
            char_size: (0, 0),
        }
    }

    pub fn new(
            ttf_context: &'a Sdl2TtfContext,
            texture_creator: &'a TextureCreator<WindowContext>,
            path: &'static str) -> Result<Self> {
        
        let mut char_textures = HashMap::new();

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

            char_textures.insert(glyph, texture);
        }

        Ok(Self {
            char_textures,
            char_size,
        })
    }

    pub fn px_to_ch_x(&self, pixels: u32) -> u32 {
        pixels / self.char_size.0
    }

    pub fn px_to_ch_y(&self, pixels: u32) -> u32 {
        pixels / self.char_size.1
    }

    pub fn ch_to_px_x(&self, chars: u32) -> u32 {
        chars * self.char_size.0
    }

    pub fn ch_to_px_y(&self, chars: u32) -> u32 {
        chars * self.char_size.1
    }

    fn draw_char(
            &mut self,
            canvas: &mut Canvas,
            pos: (u32, u32),
            ch: &'a str,
            fg: (u8, u8, u8),
            bg: Option<(u8, u8, u8)>) -> Result<()> {

        let texture = self.char_textures.get_mut(ch);

        if let Some(mut texture) = texture {
            let rect = Rect::new(
                pos.0 as i32,
                pos.1 as i32,
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

        Ok(())
    }
}

pub struct TUIDrawer {
    start_x: u32,
    start_y: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    fg: (u8, u8, u8),
}

impl TUIDrawer {
    pub fn new(
            pos: (u32, u32), 
            size: (u32, u32)) -> Self {
        
        Self {
            start_x: pos.0,
            start_y: pos.1,
            width: size.0,
            height: size.1,
            x: 0,
            y: 1,
            fg: (255, 255, 255),
        }
    }

    pub fn fill_bg(&mut self,
            canvas: &mut Canvas,
            font: &mut RenderedFont<'_>,
            bg: (u8, u8, u8)) -> Result<()> {

        canvas.set_draw_color(bg);
        canvas.fill_rect(
            Rect::new(
                self.start_x as i32,
                self.start_y as i32,
                self.width  * font.char_size.0,
                self.height * font.char_size.1
            )
        ).map_err(conv_err!())?;

        Ok(())
    }

    pub fn text_at<'a>(&mut self,
            canvas: &mut Canvas,
            font: &mut RenderedFont<'a>,
            x: u32,
            y: u32,
            text: &'a str) -> Result<()> {

        self.x = x;
        self.y = y;

        for ch in text.graphemes(true) {
            font.draw_char(
                canvas,
                (
                    self.start_x + self.x * font.char_size.0,
                    self.start_y + self.y * font.char_size.1
                ),
                ch,
                self.fg,
                None
            )?;
            
            if ch != "\n" {
                self.x += 1;
            } else {
                self.x = x;
                self.y += 1;
            }
        }

        Ok(())
    }

    pub fn text<'a>(&mut self,
            canvas: &mut Canvas,
            font: &mut RenderedFont<'a>,
            text: &'a str) -> Result<()> {

        self.text_at(canvas, font, self.x, self.y, text)
    }
}
