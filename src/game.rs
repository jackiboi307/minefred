use crate::behavior::base::*;
use crate::behavior::behaviors::*;
use crate::types::*;
use crate::{SCREEN_X, SCREEN_Y};

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

type Chunk = [[GameObject; 16]; 16];

pub struct Game {
    ecs: ECSWorld,
    entities: Vec<GameObject>,
    player: ECSEntityId,
    grid: HashMap<GridPos, Chunk>,
}

impl Game {
    pub fn new() -> Self {
        let mut s = Self{
            ecs: ECSWorld::new(),
            entities: Vec::new(),
            grid: HashMap::new(),
            player: ECSEntityId::DANGLING,
        };

        Self::init(&mut s);
        Self::generate_chunk(&mut s, GridPos::new(0, 0));

        s
    }

    fn init(&mut self) {
        self.player = self.spawn_entity(PlayerBehavior);
        self.spawn_entity(TestBehavior);
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
                            pos.x + column as PosType,
                            pos.y + row as PosType,
                        ),
                    });

                    (tile.behavior.render)
                    (&self.ecs, tile.id, &render_info, canvas);
                }
            }
        }

        render_info.tile = None;

        for entity in &self.entities {
            (entity.behavior.render)
            (&self.ecs, entity.id, &render_info, canvas);
        }

    }

    pub fn update(&mut self, update_data: &UpdateData) {
        for entity in &mut self.entities {
            (entity.behavior.update)
            (&mut self.ecs, entity.id, &update_data);
        }
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

    fn ecs_create_entity(&mut self) -> ECSEntityId {
        self.ecs.spawn(())
    }

    fn generate_chunk(&mut self, pos: GridPos) {
        let mut chunk = [[GAME_OBJECT_PLACEHOLDER; 16]; 16];
        for row in 0..16 {
            for col in 0..16 {
                chunk[row][col] = self.create_game_object(TestTileBehavior);
            }
        }
        self.grid.insert(pos, chunk);
    }
}
