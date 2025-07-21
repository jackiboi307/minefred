use crate::behavior::base::*;
use crate::behavior::behaviors::*;
use crate::types::*;
use crate::constants::{SCREEN_X, SCREEN_Y, GRID_SIZE};
use crate::textures::{Texture, TextureId, Textures, gen_texture};

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use hecs::World as ECSWorld;
use hecs::Entity as ECSEntityId;

use std::collections::HashMap;

#[derive(Copy, Clone)]
struct GameObject {
    behavior: GameObjectBehavior,
    id: ECSEntityId,
}

const GAME_OBJECT_PLACEHOLDER: GameObject = GameObject{
    behavior: DefaultBehavior,
    id: ECSEntityId::DANGLING,
};

type Chunk = [[GameObject; GRID_SIZE]; GRID_SIZE];

pub struct Game<'a> {
    ecs: ECSWorld,
    entities: Vec<GameObject>,
    player: ECSEntityId,
    grid: HashMap<GridPos, Chunk>,
    textures: Textures<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let mut s = Self{
            ecs: ECSWorld::new(),
            entities: Vec::new(),
            grid: HashMap::new(),
            textures: HashMap::new(),
            player: ECSEntityId::DANGLING,
        };

        Self::init(&mut s);
        Self::generate_chunk(&mut s, GridPos::new(0,   0));
        Self::generate_chunk(&mut s, GridPos::new(-1,  0));
        Self::generate_chunk(&mut s, GridPos::new(0,  -1));
        Self::generate_chunk(&mut s, GridPos::new(-1, -1));

        s
    }

    fn init(&mut self) {
        self.player = self.spawn_entity(PlayerBehavior);
        self.spawn_entity(TestBehavior);
    }

    pub fn init_textures(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        let texture = gen_texture(&texture_creator);
        self.register_texture("test", texture);
    }

    fn register_texture(&mut self, id: &'static str, texture: Texture<'a>) {
        self.textures.insert(TextureId(id,), texture);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let player = self.ecs.get::<&Position>(self.player).unwrap();
        let screen = Rect::new(SCREEN_X.into(), SCREEN_Y.into());
        let mut render_info = RenderInfo{
            offset: Offset::new(
                player.x - screen.width  as PosType / 2,
                player.y - screen.height as PosType / 2,
            ),
            screen,
            scale: 1.0,
            tile: None,
        };

        for (pos, chunk) in &self.grid {
            for row in 0..chunk.len() {
                for (column, tile) in chunk[row].iter().enumerate() {
                    render_info.tile = Some(TileInfo{
                        pos: GridPos::new(
                            pos.x * GRID_SIZE as i32 + column as PosType,
                            pos.y * GRID_SIZE as i32 + row as PosType,
                        ),
                    });

                    (tile.behavior.render)
                    (&self.ecs, tile.id, &render_info, &self.textures, canvas);
                }
            }
        }

        render_info.tile = None;

        for entity in &self.entities {
            (entity.behavior.render)
            (&self.ecs, entity.id, &render_info, &self.textures, canvas);
        }

    }

    pub fn update(&mut self, update_data: &UpdateData) {
        for entity in &mut self.entities {
            (entity.behavior.update)
            (&mut self.ecs, entity.id, &update_data);
        }
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
        (game_obj.behavior.init)(&mut self.ecs, game_obj.id);
        game_obj
    }

    fn spawn_entity(&mut self, behavior: GameObjectBehavior) -> ECSEntityId {
        let game_obj = self.create_game_object(behavior);
        self.entities.push(game_obj);
        game_obj.id
    }

    fn generate_chunk(&mut self, pos: GridPos) {
        let mut chunk = [[GAME_OBJECT_PLACEHOLDER; GRID_SIZE]; GRID_SIZE];
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                chunk[row][col] = self.create_game_object(TestTileBehavior);
            }
        }
        self.grid.insert(pos, chunk);
    }
}
