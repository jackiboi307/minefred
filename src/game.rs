use crate::behavior::base::*;
use crate::behavior::behaviors::*;
use crate::types::*;
use crate::constants::{SCREEN_X, SCREEN_Y, CHUNK_SIZE, RENDER_DISTANCE};
use crate::textures::{Textures, load_textures};
use crate::random;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::rect;

use hecs::World as ECSWorld;
use hecs::Entity as ECSEntityId;

use std::collections::HashMap;

#[derive(Copy, Clone)]
struct GameObject {
    behavior: GameObjectBehavior,
    id: ECSEntityId,
}

type Tile = Vec<GameObject>;
type Chunk = [[Tile; CHUNK_SIZE]; CHUNK_SIZE];

pub struct Game<'a> {
    ecs: ECSWorld,
    entities: Vec<GameObject>,
    player: ECSEntityId,
    grid: HashMap<ChunkPos, Chunk>,
    textures: Textures<'a>,
    tile_scale: u32,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let s = Self{
            ecs: ECSWorld::new(),
            entities: Vec::new(),
            grid: HashMap::new(),
            textures: HashMap::new(),
            player: ECSEntityId::DANGLING,
            tile_scale: 40,
        };

        s
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.player = self.spawn_entity(PlayerBehavior);
        self.spawn_entity(TestBehavior);

        self.generate_chunk(ChunkPos::new(0, 0))?;

        Ok(())
    }

    pub fn init_textures
            (&mut self, texture_creator: &'a TextureCreator<WindowContext>) 
            -> Result<(), Error> {
        load_textures(&texture_creator, &mut self.textures)
    }

    pub fn render(&self, canvas: &mut Canvas) -> Result<(), Error> {
        let player = self.ecs.get::<&Position>(self.player)?;
        let screen = Rect::new(SCREEN_X.into(), SCREEN_Y.into());

        let mut i = 0;
        loop {
            let mut found = false;
            for tile in self.get_loaded_tiles()? {
                let col = tile.x();
                let row = tile.y();
                let tile =
                    if let Some(chunk) = self.grid.get(&tile.chunk) {
                        &chunk[tile.chunk_y][tile.chunk_x]
                    } else {
                        continue
                    };

                if i < tile.len() {
                    found = true;
                } else {
                    continue
                }

                let render_info = RenderInfo{
                    screen,
                    rect: rect::Rect::new(
                        screen.width as i32 / 2
                            + col * self.tile_scale as i32
                            - (player.x * self.tile_scale as f32) as i32,
                        screen.height as i32 / 2
                            + row * self.tile_scale as i32
                            - (player.y * self.tile_scale as f32) as i32,
                        self.tile_scale as u32,
                        self.tile_scale as u32,
                    ),
                };

                let game_object = tile[i];
                let res = (game_object.behavior.render)
                    (&self.ecs, game_object.id, &render_info, &self.textures, canvas);
                if res.is_err() {
                    eprintln!("Error: {:?}", res);
                }
            }

            if found {
                i += 1;
            } else {
                break
            }
        }

        for entity in &self.entities {
            let pos = self.ecs.get::<&Position>(entity.id)?;
            let render_info = RenderInfo{
                screen,
                rect: rect::Rect::new(
                    ((pos.x as f32 - player.x) * self.tile_scale as f32) as i32
                        + screen.width as i32 / 2,
                    ((pos.y as f32 - player.y) * self.tile_scale as f32) as i32
                        + screen.height as i32 / 2,
                    self.tile_scale,
                    self.tile_scale,
                ),
            };
            let res = (entity.behavior.render)
                (&self.ecs, entity.id, &render_info, &self.textures, canvas);
            if res.is_err() {
                eprintln!("Error: {:?}", res);
            }
        }

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

        for chunk in self.get_loaded_chunks()? {
            if self.grid.get(&chunk).is_none() {
                self.generate_chunk(chunk)?;
            }
        }

        for entity in &mut self.entities {
            let res = (entity.behavior.update)
                (&mut self.ecs, entity.id, &update_data);
            if res.is_err() {
                eprintln!("Error: {:?}", res);
            }
        }

        Ok(())
    }

    fn get_loaded_tiles(&self)
            -> Result<[TilePos; RENDER_DISTANCE.pow(2) * CHUNK_SIZE.pow(2)], Error> {

        let loaded = self.get_loaded_chunks()?;
        let tiles:
                [TilePos; RENDER_DISTANCE.pow(2) * CHUNK_SIZE.pow(2)] =
            core::array::from_fn(|i| {
                let chunkpos = loaded[i / CHUNK_SIZE.pow(2)];
                TilePos::new(
                    chunkpos,
                    i % CHUNK_SIZE,
                    i % CHUNK_SIZE.pow(2) / CHUNK_SIZE,
                )
            });

        Ok(tiles)
    }

    fn get_loaded_chunks(&self)
            -> Result<[ChunkPos; RENDER_DISTANCE.pow(2)], Error> {

        let player = self.ecs.get::<&Position>(self.player)?.clone();
        let chunks: [ChunkPos; RENDER_DISTANCE.pow(2)] = 
            core::array::from_fn(|i|
                ChunkPos::new(
                    (player.x / 16.0).floor() as ChunkPosType
                        + (i % RENDER_DISTANCE) as ChunkPosType
                        - RENDER_DISTANCE as ChunkPosType / 2,
                    (player.y / 16.0).floor() as ChunkPosType
                        + (i / RENDER_DISTANCE) as ChunkPosType
                        - RENDER_DISTANCE as ChunkPosType / 2,
                ));

        Ok(chunks)
    }

    fn ecs_create_entity(&mut self) -> ECSEntityId {
        self.ecs.spawn(())
    }

    fn create_game_object(&mut self, behavior: GameObjectBehavior) -> GameObject {
        let id = self.ecs_create_entity();
        let game_obj = GameObject{
            behavior,
            id,
        };

        // TODO skapa inte om failar
        let res = (game_obj.behavior.init)
            (&mut self.ecs, game_obj.id, &self.textures);
        if res.is_err() {
            eprintln!("Error: {:?}", res);
        }

        game_obj
    }

    fn spawn_entity(&mut self, behavior: GameObjectBehavior) -> ECSEntityId {
        let game_obj = self.create_game_object(behavior);
        self.entities.push(game_obj);
        game_obj.id
    }

    fn generate_chunk(&mut self, pos: ChunkPos) -> Result<(), Error> {
        // let mut result = Ok(());

        let chunk: [[Tile; CHUNK_SIZE]; CHUNK_SIZE] =
            core::array::from_fn(|_| core::array::from_fn(|_| {
                let mut tile = Tile::new();
                let _ = tile.push(self.create_game_object(TestTileBehavior));
                if random::int(0..8) == 0 {
                    let _ = tile.push(self.create_game_object(TreeBehavior));
                }
                tile
            }));

        self.grid.insert(pos, chunk);

        // result
        Ok(())
    }
}
