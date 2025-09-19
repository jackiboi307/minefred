use crate::prelude::*;
use crate::utils::KeyedSlice;

use sdl2::pixels::Color;
use sdl2::ttf::{
    Sdl2TtfContext,
    Font,
};
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

// security measure in case .clear() is ever forgotten
const MAX_SNIPPETS: usize = 1000;

type Size = (u8, u8);
type Pos  = (u8, u8);

pub mod color {
    #![allow(dead_code)]

    type RGB  = (u8, u8, u8);
    type RGBA = (u8, u8, u8, u8);

    pub const WHITE:       RGB  = (255, 255, 255);
    pub const BLACK:       RGB  = (0, 0, 0);
    pub const BLUE:        RGB  = (0, 0, 255);
    pub const TRANSPARENT: RGBA = (0, 0, 0, 0);
}

macro_rules! info_methods {
    () => {
        pub fn fg(mut self, fg: impl Into<Color>) -> Self {
            self.info.fg = Some(fg.into());
            self
        }

        pub fn bg(mut self, bg: impl Into<Color>) -> Self {
            self.info.bg = Some(bg.into());
            self
        }
    };
}

struct Info {
    fg: Option<Color>,
    bg: Option<Color>,
    row: u8,
    col: u8,
}

pub struct Snippet {
    string: String,
    font: Option<&'static str>,
    info: Info,
}

pub struct Snippets {
    snippets: Vec<Snippet>,
    children: Vec<(Pos, Snippets)>,
    visible: bool,
    info: Info,
    width: u16,
    height: u16,
}

impl Info {
    pub fn new(row: u8, col: u8) -> Self {
        Self {
            row,
            col,
            fg: None,
            bg: None,
        }
    }

}

impl Snippet {
    pub fn new(string: String, row: u8, col: u8) -> Self {
        Self {
            string,
            font: None,
            info: Info::new(row, col),
        }
    }

    pub fn font(mut self, font: &'static str) -> Self {
        self.font = Some(font);
        self
    }

    info_methods!();
}

pub struct FontConfig {
    pub scale: f32,
    pub y_scale: f32,
    pub default_fg: Color,
}

impl Snippets {
    pub fn new(size: Size, row: u8, col: u8) -> Self {
        Self {
            snippets: Vec::new(),
            visible: true,
            info: Info::new(row, col),
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.snippets.clear();
        self
    }

    pub fn add(&mut self, snippet: Snippet) -> &mut Self {
        self.snippets.push(snippet);
        assert!(self.snippets.len() < MAX_SNIPPETS);
        self
    }

    info_methods!();

    // pub fn new(font: &Font) -> Self {
    //     Self {
    //         width,
    //         height,
    //         layers: Vec::new(),
    //         bg: Color::RGB(0, 0, 0),
    //         font_size: font.size_of("m").unwrap(),
    //     }
    // }

    pub fn width(&self) -> u32 {
        self.width as u32 * self.font_size.0
    }

    pub fn height(&self) -> u32 {
        self.height as u32 * self.font_size.1
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    pub fn add_layer(&mut self, layer: Snippets) {
        self.layers.push(layer);
    }

    pub fn create_layer(&mut self) -> &mut Snippets {
        self.layers.push(Layer::new());
        self.top_layer()
    }

    pub fn top_layer(&mut self) -> &mut Snippets {
        self.layers.last_mut().unwrap()
    }

    // pub fn pop_layer(&mut self) -> Option<Layer> {
    //     self.layers.pop()
    // }

    pub fn render(
            &self,
            canvas: &mut Canvas,
            fonts: &KeyedSlice<Font>,
            font_conf: FontConfig,
            pos: (i32, i32)) -> Result<()> {

        let font = fonts.by_id(0).expect("no font with id 0");

        canvas.set_draw_color(self.bg);
        canvas.fill_rect(Rect::new(
            pos.0,
            pos.1,
            self.width  as u32 * self.font_size.0,
            self.height as u32 * self.font_size.1,
        )).map_err(conv_err!())?;

        let texture_creator = (*canvas).texture_creator();

        for layer in &self.layers {
            if !layer.visible {
                continue
            }

            for snippet in &layer.snippets {
                let surface = font
                    .render(&snippet.string)
                    .blended(snippet.fg.unwrap_or_else(|| font_conf.default_fg))
                    .map_err(conv_err!())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(conv_err!())?;

                let TextureQuery { width, height, .. } = texture.query();

                let width = (width as f32 * font_conf.scale) as u32;
                let height =
                    (height as f32 * font_conf.scale * font_conf.y_scale) as u32;

                let rect = Rect::new(
                    (snippet.col * self.font_size.0) as i32 + pos.0,
                    (snippet.row * self.font_size.1) as i32 + pos.1,
                    width,
                    height
                );

                if let Some(bg) = snippet.bg {
                    canvas.set_draw_color(bg);
                    canvas.fill_rect(rect).map_err(conv_err!())?;
                }

                canvas.copy(&texture, None, rect).map_err(conv_err!())?;
            }
        }

        Ok(())
    }
}
