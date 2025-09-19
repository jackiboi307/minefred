use crate::prelude::*;
use crate::utils::KeyedSlice;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use unicode_segmentation::UnicodeSegmentation;

type Size = (u8, u8);
type Pos  = (u8, u8);
type Positioned<T> = (Pos, T);

pub mod color {
    #![allow(dead_code)]

    type RGB  = (u8, u8, u8);
    type RGBA = (u8, u8, u8, u8);

    pub const WHITE:       RGB  = (255, 255, 255);
    pub const BLACK:       RGB  = (0, 0, 0);
    pub const BLUE:        RGB  = (0, 0, 255);
    pub const RED:         RGB  = (255, 0, 0);
    pub const TRANSPARENT: RGBA = (0, 0, 0, 0);
}

enum SnippetItem {
    Single(Snippet),
    Group(SnippetGroup),
}

struct Snippet {
    string: String,
    fg: Option<Color>,
    bg: Option<Color>,
    pos: Pos,
}

pub struct SnippetGroup {
    items: Vec<Positioned<SnippetItem>>,
    default_fg: Option<Color>,
    bg: Option<Color>,
    size: Size,
}

pub struct FontInfo {
    orig_size: (u32, u32),
    scale: f32,
    scale_y: f32,
}

impl FontInfo {
    fn scaled_width(&self) -> u16 {
        (self.orig_size.0 as f32 * self.scale) as u16
    }

    fn scaled_height(&self) -> u16 {
        (self.orig_size.1 as f32 * self.scale * self.scale_y) as u16
    }
}

pub struct TUICanvas {
    root: SnippetGroup,
    font_info: FontInfo,
    default_fg: Color,
}

impl SnippetGroup {
    pub fn example() -> Self {
        Self {
            items: vec![
                ((0, 0), SnippetItem::Single(Snippet {
                    string: "test".into(),
                    fg: None,
                    bg: None,
                    pos: (0, 0),
                })),
                ((1, 1), SnippetItem::Group(Self {
                    items: vec![
                        ((0, 0), SnippetItem::Single(Snippet {
                            string: "åä".graphemes(true).nth(0).unwrap().to_string(),
                            fg: None,
                            bg: None,
                            pos: (1, 0),
                        }))
                    ],
                    default_fg: Some(color::RED.into()),
                    bg: Some(color::BLUE.into()),
                    size: (10, 10),
                })),
            ],
            default_fg: Some(color::WHITE.into()),
            bg: Some(color::BLACK.into()),
            size: (20, 20),
        }
    }

    pub fn render(
                &self,
                canvas: &mut Canvas,
                pos: (u16, u16),
                font: &Font,
                font_info: &FontInfo,
                default_fg: Color,
            ) -> Result<()> {

        if let Some(bg) = self.bg {
            canvas.set_draw_color(bg);
            canvas.fill_rect(Rect::new(
                pos.0.into(),
                pos.1.into(),
                (self.size.0 as u16 * font_info.scaled_width()).into(),
                (self.size.1 as u16 * font_info.scaled_height()).into(),
            )).map_err(conv_err!())?;
        }

        let texture_creator = (*canvas).texture_creator();

        for item in &self.items {
            let (item_pos, item) = item;
            match item {
                SnippetItem::Group(group) => {
                    group.render(
                        canvas,
                        (pos.0 + item_pos.0 as u16 * font_info.scaled_width(),
                         pos.1 + item_pos.1 as u16 * font_info.scaled_height()),
                        font,
                        font_info,
                        default_fg,
                    )?;
                },

                SnippetItem::Single(snippet) => {
                    let surface = font
                        .render(&snippet.string)
                        .blended(snippet.fg.unwrap_or_else(|| default_fg))
                        .map_err(conv_err!())?;
                    let mut texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(conv_err!())?;

                    let TextureQuery { width, height, .. } = texture.query();

                    let width = (width as f32 * font_info.scale) as u32;
                    let height =
                        (height as f32 * font_info.scale
                         * font_info.scale_y) as u32;

                    let rect = Rect::new(
                        (item_pos.0 as u16 * font_info.scaled_width()
                            + snippet.pos.0 as u16 * font_info.scaled_width()
                            + pos.0) as i32,
                        (item_pos.1 as u16 * font_info.scaled_height()
                            + snippet.pos.1 as u16 * font_info.scaled_height()
                            + pos.1 ) as i32,
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
        }

        Ok(())
    }
}

impl TUICanvas {
    pub fn new(
            root: SnippetGroup,
            font: &Font,
            font_scale: (f32, f32),
            default_fg: impl Into<Color>) -> Self {

        let font_info = FontInfo {
            orig_size: font.size_of("m").unwrap(),
            scale: font_scale.0,
            scale_y: font_scale.1,
        };

        Self {
            root,
            font_info,
            default_fg: default_fg.into(),
        }
    }

    pub fn render(&self, canvas: &mut Canvas, pos: (u16, u16), font: &Font) -> Result<()> {
        self.root.render(
            canvas,
            pos,
            font,
            &self.font_info,
            self.root.default_fg.expect("root has no default fg"),
        )
    }
}
