use super::*;
use crate::prelude::*;
use crate::components::*;
use crate::ui::tui;
use crate::textures::load_textures;

use sdl2::video::WindowContext;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::TextureCreator;

impl<'a> Game<'a> {
    pub fn init(&mut self) -> Result<()> {
        self.player = self.spawn("player", ({
            let mut inventory = Inventory::new(10);
            inventory.try_receive("test", 1);
            inventory.try_receive("error", 5);
            inventory
        },))?;

        self.spawn("player", ())?;
        let chunk = ChunkPos::new(0, 0);
        self.generate_chunk(chunk.clone())?;
        self.spawn("tree", (Position::tile(chunk.clone(), 1, 1),))?;
        self.spawn("tree", (Position::tile(chunk.clone(), 2, 1),))?;
        self.spawn("test", ())?;
        self.update_loaded(true)?;
        Ok(())
    }

    pub fn init_textures
            (&mut self, texture_creator: &'a TextureCreator<WindowContext>) 
            -> Result<()> {
        load_textures(&texture_creator, &mut self.textures)
    }

    pub fn init_fonts(
            &mut self,
            ttf_context: &'a Sdl2TtfContext,
            texture_creator: &'a TextureCreator<WindowContext>) -> Result<()> {

        // let mut fonts = KeyedSliceBuilder::<Font>::new();
        //
        // fonts.add({
        //     let mut font = ttf_context.load_font(
        //         "assets/fonts/hack_regular.ttf", 15).map_err(conv_err!())?;
        //     font.set_style(sdl2::ttf::FontStyle::BOLD);
        //     font
        // });
        //
        // self.fonts = fonts.build();
        
        self.font = tui::RenderedFont::new(ttf_context, texture_creator,
            "assets/fonts/hack_regular.ttf")?;

        Ok(())
    }
}
