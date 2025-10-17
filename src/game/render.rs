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

        self.render_ui(canvas).context("rendering ui")?;

        Ok(())
    }

    pub fn render_ui(&mut self, canvas: &mut Canvas) -> Result<()> {
        use crate::ui::*;

        if let Some(inventory) = self.ui_handler.get("inventory") {
            if inventory.visible {
                let mut drawer = tui::TUIDrawer::new(inventory.to_rect(self.screen_size));
                drawer.fill_bg(canvas, &mut self.font, (0, 0, 0))?;
                if let Ok(inventory) = self.ecs.get::<&Inventory>(self.player) {
                    for (i, item) in inventory.items.iter().enumerate() {
                        if let Some(item) = item {
                            let text = format!("{}: {} ({})\n", i, item.key, item.amount);
                            drawer.text(canvas, &mut self.font, text.into())?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
