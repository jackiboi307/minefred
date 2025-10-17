mod init;
mod render;
mod update;

use crate::prelude::*;
use crate::utils::Counter;
use crate::gameobjtype::*;
use crate::components::*;
use crate::event::ActionHandler;
use crate::constants::*;
use crate::textures::Textures;
use crate::ui::UIHandler;
use crate::ui::tui;

use sdl2::rect::Rect;

use hecs::World as ECSWorld;
use hecs::Entity as EntityId;
use hecs::{
    DynamicBundle,
    EntityBuilder
};

use std::collections::HashMap;

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

gen_struct! { pub Game<'a> {
    ecs: ECSWorld = ECSWorld::new(),
    types: GameObjectTypes = GameObjectTypes::generate(),
    textures: Textures<'a> = HashMap::new(),
    font: tui::RenderedFont<'a> = tui::RenderedFont::empty(),
    loaded: Loaded = Loaded::new(),
    loaded_update_counter: Counter = Counter::new(60),
    player: EntityId = EntityId::DANGLING,
    chunks: Vec<ChunkPos> = Vec::new(),
    player_chunk: ChunkPos = ChunkPos::new(0, 0),
    tile_scale: u32 = 40,
    screen_size: (i32, i32) = (0, 0),
    action_handler: ActionHandler = ActionHandler::new(),
    ui_handler: UIHandler = UIHandler::new(),
    // last_mouse_pos: (i32, i32) = (0, 0),
} pub new }

impl<'a> Game<'a> {
    // pub fn new() -> Self {
    //     let s = Self {
    //         ecs: ECSWorld::new(),
    //         types: GameObjectTypes::generate(),
    //         textures: HashMap::new(),
    //         font: tui::RenderedFont::empty(),
    //         loaded: Loaded::new(),
    //         loaded_update_counter: Counter::new(60),
    //         player: EntityId::DANGLING,
    //         chunks: Vec::new(),
    //         player_chunk: ChunkPos::new(0, 0),
    //         tile_scale: 40,
    //         screen_size: (0, 0),
    //         action_handler: ActionHandler::new(),
    //         ui_handler: UIHandler::new(),
    //         last_mouse_pos: (0, 0),
    //     };
    //
    //     s
    // }

    fn get_gameobjtype(&self, id: EntityId) -> &GameObjectType {
        let type_id = self.ecs.get::<&GameObjectTypeComponent>(id).unwrap();
        self.types.from_id(type_id.id)
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
