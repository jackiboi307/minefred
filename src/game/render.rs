use super::*;
use crate::prelude::*;
use crate::textures::copy_texture;
use crate::components::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl<'a> Game<'a> {
    pub fn render(&mut self, canvas: &mut Canvas) -> Result<()> {
        let timer = debug::Timer::new("rendering");
        for id in &self.loaded.ids {
            let rect = if let Ok(rect) = self.get_sdl_rect(*id) {
                rect } else { continue };

            if let Some(texture_id) = self.get_gameobjtype(*id).texture {
                copy_texture(
                    canvas,
                    &self.textures,
                    texture_id,
                    self.ecs.get::<&TextureTransform>(*id).ok().as_deref(),
                    rect
                ).map_err(conv_err!())?;
            }
        }
        timer.done();

        if let Ok(player) = self.ecs.get::<&Player>(self.player) {
            if let Some(selected) = player.selected {
                if let Ok(rect) = self.get_sdl_rect(selected) {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.draw_lines([
                        rect.top_left(),
                        rect.top_right(),
                        rect.bottom_right(),
                        rect.bottom_left(),
                        rect.top_left(),
                    ].as_slice()).map_err(conv_err!())?;
                }
            }
        }

        self.render_tui(canvas).context("rendering tui")?;

        Ok(())
    }

    pub fn render_tui(&mut self, canvas: &mut Canvas) -> Result<()> {
        #[allow(unused_imports)]
        use crate::ui::tui::{
        };

        let width = std::cmp::min(
            self.font.pixels_to_chars_x(self.screen_size.0 as u32 / 2),
            40);
        let height = std::cmp::min(
            self.font.pixels_to_chars_y(self.screen_size.1 as u32 / 2),
            40);

        let x = (self.font.pixels_to_chars_x(self.screen_size.0 as u32) / 2)
            .saturating_sub(width / 2);
        let y = (self.font.pixels_to_chars_y(self.screen_size.1 as u32) / 2)
            .saturating_sub(height / 2);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            self.font.chars_to_pixels_x(x) as i32,
            self.font.chars_to_pixels_y(y) as i32,
            self.font.chars_to_pixels_x(width),
            self.font.chars_to_pixels_y(height)
        )).map_err(conv_err!())?;

        if let Ok(inventory) = self.ecs.get::<&Inventory>(self.player) {
            for (i, item) in inventory.items.iter().enumerate() {
                if let Some(item) = item {
                    self.font.render_text(canvas,
                        (x, y + i as u32),
                        (width, height),
                        format!("{} ({})", item.key, item.amount),
                        (255, 255, 255),
                        None,
                    )?;
                }
            }
        }
        
        Ok(())

        /*
        use tui::color::*;
        use tui::{
            Snippet,
            TUICanvas,
        };
        
        let mut tuicanvas = TUICanvas::new(20, 20, self.fonts.by_id(0).unwrap());
        let layer = tuicanvas.create_layer();
        layer.clear();
        layer.add(Snippet::new("Inventory".into(), 0, 0));

        let selected = 0;

        if let Ok(inventory) = self.ecs.get::<&Inventory>(self.player) {
            for (i, item) in inventory.items.iter().enumerate() {
                if let Some(item) = item {
                    layer.add(Snippet::new(
                        format!("{} ({})", item.key, item.amount),
                        i as u32 + 1, 4)
                        .fg(if i == selected { BLACK } else { WHITE })
                        .bg(if i == selected { WHITE } else { BLACK })
                    );
                }
            }
        }

        let font_config = tui::FontConfig {
            scale: 1.0,
            y_scale: 0.9,
            default_fg: WHITE.into(),
        };

        let x = (self.screen_size.0 - tuicanvas.width()  as i32) / 2;
        let y = (self.screen_size.1 - tuicanvas.height() as i32) / 2;
        tuicanvas.render(
            canvas,
            &self.fonts,
            font_config,
            (x, y),
        ).context("rendering tui")?;
        */

        /*
        let font = self.fonts.by_id(0).unwrap();
        let example = tui::SnippetGroup::example();
        let tuicanvas = tui::TUICanvas::new(example, font, (1.0, 1.0), tui::color::WHITE);
        tuicanvas.render(canvas, (100, 100), font)?;
        */
    }
}
