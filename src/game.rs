use crate::gameobjtype::*;
use crate::components::*;
use crate::event::EventHandler;
use crate::types::*;
use crate::constants::*;
use crate::utils::*;
use crate::textures::{Textures, load_textures, copy_texture};
use crate::debug;

use sdl2::render::TextureCreator;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::rect;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::{
    DynamicBundle,
    EntityBuilder
};

use std::collections::HashMap;
use std::cmp::Ordering;

fn handle_err(label: &str, res: Result<(), impl std::fmt::Display>) {
    if let Err(res) = res {
        eprintln!("error: {}: {}", label, res);
    }
}

struct Loaded {
    ids: Vec<EntityId>,
}

impl Loaded {
    fn new() -> Self {
        Self{
            ids: Vec::new(),
        }
    }
}

pub struct Game<'a> {
    ecs: ECSWorld,
    types: GameObjectTypes,
    textures: Textures<'a>,
    loaded: Loaded,
    loaded_update_counter: Counter,
    player: EntityId,
    chunks: Vec<ChunkPos>,
    player_chunk: ChunkPos,
    tile_scale: u32,
    screen: Rect,
    event_handler: EventHandler,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let s = Self{
            ecs: ECSWorld::new(),
            types: GameObjectTypes::generate(),
            textures: HashMap::new(),
            loaded: Loaded::new(),
            loaded_update_counter: Counter::new(60),
            player: EntityId::DANGLING,
            chunks: Vec::new(),
            player_chunk: ChunkPos::new(0, 0),
            tile_scale: 40,
            screen: Rect::new(SCREEN_X.into(), SCREEN_Y.into()),
            event_handler: EventHandler::new(),
        };

        s
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.player = self.spawn("player", ())?;
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
            -> Result<(), Error> {
        load_textures(&texture_creator, &mut self.textures)
    }

    pub fn render(&self, canvas: &mut Canvas) -> Result<(), Error> {
        let timer = debug::Timer::new("rendering");
        for id in &self.loaded.ids {
            let rect = if let Ok(rect) = self.get_sdl_rect(*id) {
                rect } else { continue };

            if let Ok(texture) = self.ecs.get::<&Texture>(*id) {
                copy_texture(canvas, &self.textures, &texture, rect)?;
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
                    ].as_slice())?;
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self, event_pump: &mut EventPump) -> Result<bool, Error> {
        let timer = debug::Timer::new("handling events");

        self.event_handler.reset();

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
                        self.event_handler.register_event(scancode, true); }
                },
                Event::KeyUp { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        self.event_handler.register_event(scancode, false); }
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.event_handler.register_event(mouse_btn, true);
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.event_handler.register_event(mouse_btn, false);
                },
                _ => {}
            }
        }

        let update_data = UpdateData{};
        timer.done();

        self.update_loaded(false)?;

        self.update_player();

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

        let timer = debug::Timer::new("updating");
        for (id, update_fn) in id_update_fn_pairs {
            handle_err(&format!("updating entity {}", id.id()).to_string(),
                update_fn(&mut self.ecs, id, &update_data));
        }
        timer.done();

        Ok(false)
    }

    fn update_player(&mut self) {
        let speed =
            if self.event_handler.key("run")
                { 0.2 } else { 0.1 };

        if let Ok(mut pos) = self.ecs.get::<&mut Position>(self.player) {
            if self.event_handler.key("move_right") { pos.move_x(speed); }
            if self.event_handler.key("move_left")  { pos.move_x(-speed); }
            if self.event_handler.key("move_down")  { pos.move_y(speed); }
            if self.event_handler.key("move_up")    { pos.move_y(-speed); }
        }

        let selected =
            if self.event_handler.key("attack") {
                if let Ok(player) = self.ecs.get::<&Player>(self.player) {
                    player.selected
                } else { None }
            } else { None };

        if let Some(selected) = selected {
            handle_err("despawning selected entity",
                self.ecs.despawn(selected).into());
        }
    }

    fn get_sdl_rect(&self, id: EntityId) -> Result<rect::Rect, Error> {
        let pos = if let Ok(pos) = self.ecs.get::<&Position>(id) {
            pos } else { return Err("no position component".into()) };

        let player = self.ecs.get::<&Position>(self.player)?;

        let rect = if pos.is_free() {
            rect::Rect::new(
                ((pos.x() as f32 - player.x()) * self.tile_scale as f32) as i32
                    + self.screen.width as i32 / 2,
                ((pos.y() as f32 - player.y()) * self.tile_scale as f32) as i32
                    + self.screen.height as i32 / 2,
                self.tile_scale,
                self.tile_scale,
            )

        } else {
            rect::Rect::new(
                self.screen.width as i32 / 2
                    + (pos.x() * self.tile_scale as PosType) as i32
                    - (player.x() * self.tile_scale as f32) as i32,
                self.screen.height as i32 / 2
                    + (pos.y() * self.tile_scale as PosType) as i32
                    - (player.y() * self.tile_scale as f32) as i32,
                self.tile_scale,
                self.tile_scale,
            )
        };

        Ok(rect)
    }

    fn update_loaded(&mut self, force: bool) -> Result<(), Error> {
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
            -> Result<EntityId, Error> {

        let mut builder = EntityBuilder::new();
        builder.add_bundle(components);
        self.types.init_entity(&mut builder, type_key)?;
        let entity = builder.build();
        let id = self.ecs.spawn(entity);
        Ok(id)
    }

    fn generate_chunk
            (&mut self, pos: ChunkPos) -> Result<(), Error> {

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
