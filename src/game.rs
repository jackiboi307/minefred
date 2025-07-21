use crate::behavior::base::*;
use crate::behavior::behaviors::*;
use crate::types::*;
use crate::{SCREEN_X, SCREEN_Y};

use hecs::World as ECSWorld;
use hecs::Entity as ECSEntityId;

struct GameObject {
    behavior: GameObjectBehavior,
    id: ECSEntityId,
}

pub struct Game {
    ecs: ECSWorld,
    entities: Vec<GameObject>,
    player: ECSEntityId,
}

impl Game {
    pub fn new() -> Self {
        let mut s = Self{
            ecs: ECSWorld::new(),
            entities: Vec::new(),
            player: ECSEntityId::DANGLING,
        };

        Self::init(&mut s);

        s
    }

    fn init(&mut self) {
        self.player = self.spawn_entity(PlayerBehavior);
        self.spawn_entity(TestBehavior);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let player = self.ecs.get::<&Position>(self.player).unwrap();
        let screen = Rect::new(SCREEN_X.into(), SCREEN_Y.into());
        let render_info = RenderInfo{
            offset: Offset::new(
                player.x - screen.width  as PosType / 2,
                player.y - screen.height as PosType / 2,
            ),
            screen,
            scale: 1.0,
        };

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

    fn spawn_entity(&mut self, behavior: GameObjectBehavior) -> ECSEntityId {
        let id = self.ecs_create_entity();
        let go = GameObject{
            behavior,
            id,
        };

        (go.behavior.init)(&mut self.ecs, id);
        self.entities.push(go);

        id
    }

    fn ecs_create_entity(&mut self) -> ECSEntityId {
        self.ecs.spawn(())
    }
}
