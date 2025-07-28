use crate::behavior::base::*;
use crate::behavior::behaviors::*;
use crate::types::*;
use crate::constants::*;
use crate::textures::{Textures, load_textures};
use crate::debug;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::rect;

use hecs::World as ECSWorld;
use hecs::Entity as ECSEntityId; // TODO byt namn
use hecs::DynamicBundle;

use std::collections::HashMap;
use std::cmp::Ordering;

struct Loaded {
    ids: Vec<ECSEntityId>,
}

impl Loaded {
    fn new() -> Self {
        Self{
            ids: Vec::new(),
        }
    }
}

struct Counter {
    i: u8,
    max: u8,
}

impl Counter {
    fn new(max: u8) -> Self {
        Self{i: max, max}
    }

    fn count(&mut self) -> bool {
        if self.i == self.max {
            self.i = 0;
            true
        } else {
            self.i += 1;
            false
        }
    }
}

pub struct Game<'a> {
    ecs: ECSWorld,
    player: ECSEntityId,
    textures: Textures<'a>,
    behaviors: HashMap<ECSEntityId, GameObjectBehavior>,
    loaded: Loaded,
    loaded_update_counter: Counter,
    chunks: Vec<ChunkPos>,
    player_chunk: ChunkPos,
    tile_scale: u32,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let s = Self{
            ecs: ECSWorld::new(),
            player: ECSEntityId::DANGLING,
            textures: HashMap::new(),
            behaviors: HashMap::new(),
            loaded: Loaded::new(),
            loaded_update_counter: Counter::new(60),
            chunks: Vec::new(),
            player_chunk: ChunkPos::new(0, 0),
            tile_scale: 40,
        };

        s
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.player = self.spawn(PlayerBehavior, ());

        let chunk = ChunkPos::new(0, 0);

        self.generate_chunk(chunk.clone())?;

        self.spawn(TestBehavior, (Position::tile(chunk.clone(), 0, 0),));

        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 1, 1), ZIndex::new(1)));
        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 1, 0), ZIndex::new(1)));
        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 2, 1), ZIndex::new(1)));
        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 4, 1), ZIndex::new(1)));
        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 4, 2), ZIndex::new(1)));
        self.spawn(TreeBehavior, (Position::tile(chunk.clone(), 4, 3), ZIndex::new(1)));

        self.update_loaded(true)?;

        Ok(())
    }

    pub fn init_textures
            (&mut self, texture_creator: &'a TextureCreator<WindowContext>) 
            -> Result<(), Error> {
        load_textures(&texture_creator, &mut self.textures)
    }

    fn update_loaded(&mut self, force: bool) -> Result<(), Error> {
        if !force && !self.loaded_update_counter.count() {
            return Ok(())
        }

        let do_update = force ||
            self.ecs.get::<&Position>(self.player)?.clone().chunk() != self.player_chunk;

        if do_update {
            self.player_chunk = self.ecs.get::<&Position>(self.player)?.chunk();

            // let timer = debug::Timer::new("calculating chunks");
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
            // timer.done();

            let timer = debug::Timer::new("generating chunks?");
            for chunk in &chunks {
                if !self.chunks.contains(chunk) {
                    self.generate_chunk(chunk.clone())?;
                }
            }
            timer.done();
            
            let timer = debug::Timer::new("getting ids");
            let mut render_order: [Vec<ECSEntityId>; 5] =
                core::array::from_fn(|_| Vec::new());
            for (id, (pos,)) in self.ecs.query::<(&Position,)>().iter() {
                if chunks.contains(&pos.chunk()) {
                    let has_zindex = self.ecs.get::<&ZIndex>(id).is_ok();
                    render_order[
                        if id == self.player {
                            2
                        } else {
                            match pos.pos {
                                PosKind::Free{ .. } => {
                                    if has_zindex { 4 } else { 1 }
                                },

                                PosKind::Tile{ .. } => {
                                    if has_zindex { 3 } else { 0 }
                                }
                            }
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

    pub fn render(&self, canvas: &mut Canvas) -> Result<(), Error> {
        let player = self.ecs.get::<&Position>(self.player)?;
        let screen = Rect::new(SCREEN_X.into(), SCREEN_Y.into());

        // let timer = debug::Timer::new("rendering");
        for id in &self.loaded.ids {
            let rect =
                if let Ok(pos) = self.ecs.get::<&Position>(*id) {
                    match pos.pos {
                        PosKind::Free{ .. } => rect::Rect::new(
                            ((pos.x() as f32 - player.x()) * self.tile_scale as f32) as i32
                                + screen.width as i32 / 2,
                            ((pos.y() as f32 - player.y()) * self.tile_scale as f32) as i32
                                + screen.height as i32 / 2,
                            self.tile_scale,
                            self.tile_scale,
                        ),

                        PosKind::Tile{ .. } => rect::Rect::new(
                            screen.width as i32 / 2
                                + (pos.x() * self.tile_scale as PosType) as i32
                                - (player.x() * self.tile_scale as f32) as i32,
                            screen.height as i32 / 2
                                + (pos.y() * self.tile_scale as PosType) as i32
                                - (player.y() * self.tile_scale as f32) as i32,
                            self.tile_scale,
                            self.tile_scale,
                        )
                    }

                } else {
                    continue
                };

            let render_info = RenderInfo{
                screen,
                rect,
            };

            let res =
                (self.get_behavior(*id).render)
                (&self.ecs, *id, &render_info, &self.textures, canvas);
            if res.is_err() {
                eprintln!("Error: {:?}", res);
            }
        }
        // timer.done();

        Ok(())
    }

    pub fn update(&mut self, update_data: &UpdateData) -> Result<(), Error> {
        for event in &update_data.events {
            match event {
                Event::MouseWheel { y, .. } => {
                    self.tile_scale =
                        (self.tile_scale as i32).saturating_sub(*y).min(70).max(30) as u32;
                },
                _ => {}
            }
        }

        self.update_loaded(false)?;

        // let timer = debug::Timer::new("updating");
        for id in &self.loaded.ids {
            let res =
                (self.get_behavior(*id).update)
                (&mut self.ecs, *id, &update_data);
            if res.is_err() {
                eprintln!("Error: {:?}", res);
            }
        }
        // timer.done();

        Ok(())
    }

    fn get_behavior(&self, id: ECSEntityId) -> GameObjectBehavior {
        *self.behaviors.get(&id).unwrap_or_else(|| &DefaultBehavior)
    }

    fn spawn
            (&mut self, behavior: GameObjectBehavior, components: impl DynamicBundle)
            -> ECSEntityId {
        let id = self.ecs.spawn(components);
        self.behaviors.insert(id, behavior);
        let res =
            (self.get_behavior(id).init)
            (&mut self.ecs, id, &self.textures);
        if res.is_err() {
            eprintln!("Error: {:?}", res);
        }
        id
    }

    fn generate_chunk
            (&mut self, pos: ChunkPos) -> Result<(), Error> {

        for col in 0..CHUNK_SIZE {
            for row in 0..CHUNK_SIZE {
                self.spawn(TestTileBehavior, (
                    Position::tile(pos.clone(), col as u8, row as u8),
                ));
            }
        }

        self.chunks.push(pos);

        Ok(())
    }
}
