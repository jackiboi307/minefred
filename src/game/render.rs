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
        let size = (
            std::cmp::min(self.font.px_to_ch_x(self.screen_size.0 as u32 / 2), 40),
            std::cmp::min(self.font.px_to_ch_y(self.screen_size.1 as u32 / 2), 40)
        );

        let pos = (
            (self.screen_size.0 as u32 / 2)
                .saturating_sub(self.font.ch_to_px_x(size.0) / 2),
            (self.screen_size.1 as u32 / 2)
                .saturating_sub(self.font.ch_to_px_y(size.1) / 2)
        );

        let mut drawer = tui::TUIDrawer::new(pos, size);
        drawer.fill_bg(canvas, &mut self.font, (0, 0, 0))?;
        drawer.text_at(canvas, &mut self.font, 1, 1, "hej\nhur gåre")?;
        drawer.text(canvas, &mut self.font, "jo tack")?;

        Ok(())
    }
}
