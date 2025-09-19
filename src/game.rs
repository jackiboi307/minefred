use crate::prelude::*;
use crate::gameobjtype::*;
use crate::components::*;
use crate::event::{ActionHandler, ActionUpdates};
use crate::constants::*;
use crate::utils::*;
use crate::textures::{Textures, load_textures, copy_texture};
use crate::ui::tui;
use crate::debug;

use sdl2::render::{
    TextureCreator,
    // TextureQuery,
};
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::event::{
    Event,
    WindowEvent,
};
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::ttf::Sdl2TtfContext;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::{
    DynamicBundle,
    EntityBuilder
};

use std::collections::HashMap;
use std::cmp::Ordering;

fn handle_err(label: &str, res: std::result::Result<(), impl std::fmt::Display>) {
    if let Err(res) = res {
        eprintln!("error: {}: {}", label, res);
    }
}

struct Loaded {
    ids: Vec<EntityId>,
}

impl Loaded {
    fn new() -> Self {
        Self {
            ids: Vec::new(),
        }
    }
}

pub struct Game<'a> {
    ecs: ECSWorld,
    types: GameObjectTypes,
    textures: Textures<'a>,
    font: tui::RenderedFont<'a>,
    loaded: Loaded,
    loaded_update_counter: Counter,
    player: EntityId,
    chunks: Vec<ChunkPos>,
    player_chunk: ChunkPos,
    tile_scale: u32,
    screen_size: (i32, i32),
    action_handler: ActionHandler,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let s = Self {
            ecs: ECSWorld::new(),
            types: GameObjectTypes::generate(),
            textures: HashMap::new(),
            font: tui::RenderedFont::empty(),
            loaded: Loaded::new(),
            loaded_update_counter: Counter::new(60),
            player: EntityId::DANGLING,
            chunks: Vec::new(),
            player_chunk: ChunkPos::new(0, 0),
            tile_scale: 40,
            screen_size: (SCREEN_X.into(), SCREEN_Y.into()),
            action_handler: ActionHandler::new(),
        };

        s
    }

    pub fn init(&mut self) -> Result<()> {
        self.player = self.spawn("player", ({
            let mut inventory = Inventory::new(100);
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

    fn get_gameobjtype(&self, id: EntityId) -> &GameObjectType {
        let type_id = self.ecs.get::<&GameObjectTypeComponent>(id).unwrap();
        self.types.from_id(type_id.id)
    }

    pub fn render(&self, canvas: &mut Canvas) -> Result<()> {
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

        Ok(())
    }

    pub fn render_tui(&mut self, canvas: &mut Canvas) -> Result<()> {
        use crate::ui::tui::tui_input;
        use crate::ui::tui::Fg;

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

        let input = tui_input!(
            "hej!\nhur mås det? ",
            Fg(255, 0, 0),
            "text i rött",
            Fg(255, 255, 255),
            "vääääääääldigt lång text\n",
            "ny rad",
        );

        self.font.render_text(canvas, (x, y), (width, height), (1.0, 1.0), input)?;

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

    pub fn update(&mut self, event_pump: &mut EventPump) -> Result<bool> {
        let timer = debug::Timer::new("handling events");
        let mut updates = ActionUpdates::new();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return Ok(true);
                },
                Event::MouseWheel { y, .. } => {
                    self.tile_scale =
                        (self.tile_scale as i32).saturating_sub(y).min(70).max(30) as u32;
                },
                Event::KeyDown { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        updates.register_event(&self.action_handler, scancode, true); }
                },
                Event::KeyUp { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        updates.register_event(&self.action_handler, scancode, false); }
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    updates.register_event(&self.action_handler, mouse_btn, true);
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    updates.register_event(&self.action_handler, mouse_btn, false);
                },
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            self.screen_size = (width, height);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        timer.done();

        if let Ok(mut player) = self.ecs.get::<&mut Player>(self.player) {
            player.action_state.update(&self.action_handler, &updates);
        }

        self.update_loaded(false)?;

        self.update_player()?;

        if let Ok(mut player) = self.ecs.get::<&mut Player>(self.player) {
            player.selected = 'block: {
                let mouse = event_pump.mouse_state();

                for i in (0..self.loaded.ids.len()).rev() {
                    let id = self.loaded.ids[i];
                    if id == self.player { continue }
                    if let Ok(rect) = self.get_sdl_rect(id) {
                        if rect.contains_point((mouse.x(), mouse.y())) {
                            break 'block Some(id);
                        }
                    }
                }

                None
            };
        }

        let timer = debug::Timer::new("getting update fns");
        let mut id_update_fn_pairs = Vec::new();
        for id in &self.loaded.ids {
            if let Ok(update_fn) = self.ecs.get::<&UpdateFn>(*id) {
                id_update_fn_pairs.push((
                    *id,
                    self.types.get_update_fn_from_id(update_fn.id)
                ));
            }
        }
        timer.done();

        let update_data = UpdateData{};

        let timer = debug::Timer::new("updating");
        for (id, update_fn) in id_update_fn_pairs {
            handle_err(&format!("updating entity {}", id.id()).to_string(),
                update_fn(&mut self.ecs, id, &update_data));
        }
        timer.done();

        Ok(false)
    }

    fn update_player(&mut self) -> Result<()> {
        let actions = self.ecs.get::<&Player>(self.player)?.action_state.clone();

        let speed =
            if actions.key("run")
                { 0.2 } else { 0.1 };

        if let Ok(mut pos) = self.ecs.get::<&mut Position>(self.player) {
            if actions.key("move_right") { pos.move_x(speed); }
            if actions.key("move_left")  { pos.move_x(-speed); }
            if actions.key("move_down")  { pos.move_y(speed); }
            if actions.key("move_up")    { pos.move_y(-speed); }
        }

        let selected =
            if actions.key("attack") {
                if let Ok(player) = self.ecs.get::<&Player>(self.player) {
                    player.selected
                } else { None }
            } else { None };

        if let Some(selected) = selected {
            handle_err("despawning selected entity",
                self.ecs.despawn(selected).into());
        }

        Ok(())
    }

    fn get_sdl_rect(&self, id: EntityId) -> Result<Rect> {
        let pos = if let Ok(pos) = self.ecs.get::<&Position>(id) {
            pos
        } else {
            bail!("no position component");
        };

        let player = self.ecs.get::<&Position>(self.player)?;

        let pos = if pos.is_free() {(
            ((pos.x() as f32 - player.x()) * self.tile_scale as f32) as i32
                + self.screen_size.0 as i32 / 2,
            ((pos.y() as f32 - player.y()) * self.tile_scale as f32) as i32
                + self.screen_size.1 as i32 / 2,

        )} else {(
            self.screen_size.0 as i32 / 2
                + (pos.x() * self.tile_scale as PosType) as i32
                - (player.x() * self.tile_scale as f32) as i32,
            self.screen_size.1 as i32 / 2
                + (pos.y() * self.tile_scale as PosType) as i32
                - (player.y() * self.tile_scale as f32) as i32,
        )};

        let rect = Rect::new(
            pos.0 - self.tile_scale as i32 / 2,
            pos.1 - self.tile_scale as i32 / 2,
            self.tile_scale,
            self.tile_scale,
        );

        Ok(rect)
    }

    fn update_loaded(&mut self, force: bool) -> Result<()> {
        if !force && !self.loaded_update_counter.count() {
            return Ok(())
        }

        let do_update = force ||
            self.ecs.get::<&Position>(self.player)?.clone().chunk() != self.player_chunk;

        if do_update {
            self.player_chunk = self.ecs.get::<&Position>(self.player)?.chunk();

            let chunks = {
                let player_chunk = self.ecs.get::<&Position>(self.player)?.chunk();
                core::array::from_fn::<ChunkPos, {RENDER_DISTANCE.pow(2)}, _>(|i|
                    ChunkPos::new(
                        player_chunk.x
                            + (i % RENDER_DISTANCE) as ChunkPosType
                            - RENDER_DISTANCE as ChunkPosType / 2,
                        player_chunk.y
                            + (i / RENDER_DISTANCE) as ChunkPosType
                            - RENDER_DISTANCE as ChunkPosType / 2,
                )).to_vec()
            };

            let timer = debug::Timer::new("generating chunks?");
            for chunk in &chunks {
                if !self.chunks.contains(chunk) {
                    self.generate_chunk(chunk.clone())?;
                }
            }
            timer.done();
            
            let timer = debug::Timer::new("getting ids");

            let mut render_order: [Vec<EntityId>; 5] =
                core::array::from_fn(|_| Vec::new());

            for (id, (pos,)) in self.ecs.query::<(&Position,)>().iter() {
                if chunks.contains(&pos.chunk()) {
                    render_order[
                        if id == self.player {
                            2
                        } else {
                            pos.order()
                        }
                    ].push(id);
                }
            }

            render_order[3].sort_unstable_by(|id1, id2| {
                let pos1 = self.ecs.get::<&Position>(*id1).unwrap();
                let pos2 = self.ecs.get::<&Position>(*id2).unwrap();

                let pos1 = pos1.x() + pos1.y();
                let pos2 = pos2.x() + pos2.y();

                if pos1 < pos2 {
                    Ordering::Less 
                } else {
                    Ordering::Greater
                }
            });

            let ids = render_order.iter().flat_map(|v| v.iter()).cloned().collect();

            timer.done();

            self.loaded = Loaded{
                ids,
            };
        }

        Ok(())
    }

    fn spawn
            (&mut self, type_key: &'static str, components: impl DynamicBundle)
            -> Result<EntityId> {

        let mut builder = EntityBuilder::new();
        builder.add_bundle(components);
        self.types.init_entity(&mut builder, type_key)?;
        let entity = builder.build();
        let id = self.ecs.spawn(entity);
        Ok(id)
    }

    fn generate_chunk
            (&mut self, pos: ChunkPos) -> Result<()> {

        for col in 0..CHUNK_SIZE {
            for row in 0..CHUNK_SIZE {
                self.spawn("test_tile", (
                    Position::tile(pos.clone(), col as u8, row as u8),
                ))?;
            }
        }

        self.chunks.push(pos);

        Ok(())
    }
}
