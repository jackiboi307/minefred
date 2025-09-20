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

#[macro_export]
macro_rules! tui_input {
    ( $( $e:expr ),* $(,)? ) => {{
        let mut vec = Vec::<crate::ui::tui::RenderTextSegment>::new();
        $(
            vec.push($e.into());
        )*
        vec
    }};
}

pub use crate::tui_input;

pub struct Fg(pub u8, pub u8, pub u8);
pub struct Bg(pub u8, pub u8, pub u8);
pub struct BgNone;

pub enum RenderTextSegment {
    AddString(Box<str>),
    SetColorFg(Fg),
    SetColorBg(Option<Bg>),
}

impl<T> From<T> for RenderTextSegment
where T: Into<Box<str>> {
    fn from(string: T) -> Self {
        Self::AddString(string.into())
    }
}

impl From<Fg> for RenderTextSegment {
    fn from(fg: Fg) -> Self {
        Self::SetColorFg(fg)
    }
}

impl From<Bg> for RenderTextSegment {
    fn from(bg: Bg) -> Self {
        Self::SetColorBg(Some(bg))
    }
}

impl From<BgNone> for RenderTextSegment {
    fn from(_: BgNone) -> Self {
        Self::SetColorBg(None)
    }
}

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
            font_scale: (f32, f32),
            input: Vec<RenderTextSegment>) -> Result<()> {

        let char_size = (
            (self.char_size.0 as f32 * font_scale.0) as u32,
            (self.char_size.1 as f32 * font_scale.0 * font_scale.1) as u32
        );

        let mut fg = Fg(255, 255, 255);
        let mut bg: Option<Bg> = None;
        let mut x = 0;
        let mut y = 0;

        for segment in input {
            if let RenderTextSegment::SetColorFg(new_fg) = segment {
                fg = new_fg;

            } else if let RenderTextSegment::SetColorBg(new_bg) = segment {
                bg = new_bg;

            } else if let RenderTextSegment::AddString(string) = segment {
                for glyph in string.graphemes(true) {
                    let texture = self.glyph_textures.get_mut(glyph);

                    if let Some(mut texture) = texture {
                        if (x as u32) < size.0 {
                            let rect = Rect::new(
                                (pos.0 as i32 + x) * char_size.0 as i32,
                                (pos.1 as i32 + y) * char_size.1 as i32,
                                char_size.0,
                                char_size.1
                            );

                            if let Some(ref bg) = bg {
                                canvas.set_draw_color((bg.0, bg.1, bg.2));
                                canvas.fill_rect(rect)
                                    .map_err(conv_err!())?;
                            }

                            texture.set_color_mod(fg.0, fg.1, fg.2);
                            canvas.copy(&texture, None, rect).map_err(conv_err!())?;
                        }
                    }

                    if glyph != "\n" {
                        x += 1;
                    } else {
                        x = 0;
                        y += 1;
                        if y as u32 == size.1 {
                            return Ok(())
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
