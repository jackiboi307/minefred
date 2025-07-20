use crate::behaviors::*;
use crate::update::UpdateData;

use hecs::World as ECSWorld;
use hecs::Entity as ECSEntityId;

struct GameObject {
    behavior: Box<dyn GameObjectBehavior>,
    id: ECSEntityId,
}

pub struct Game {
    ecs: ECSWorld,
    entities: Vec<GameObject>,
}

impl Game {
    pub fn new() -> Self {
        Self{
            ecs: ECSWorld::new(),
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.spawn_entity(TestBehavior{});
    }

    pub fn render(&self, canvas: &mut Canvas) {
        for entity in &self.entities {
            entity.behavior.render(&self.ecs, entity.id, canvas);
        }
    }

    pub fn update(&mut self, update_data: &UpdateData) {
        for entity in &mut self.entities {
            entity.behavior.update(&mut self.ecs, entity.id, &update_data);
        }
    }

    fn spawn_entity
            <T: GameObjectBehavior + 'static>
            (&mut self, behavior: T) -> ECSEntityId {
    
        let id = self.ecs_create_entity();
        let go = GameObject{
            behavior: Box::new(behavior),
            id,
        };
        go.behavior.init(&mut self.ecs, id);
        self.entities.push(go);

        id
    }

    fn ecs_create_entity(&mut self) -> ECSEntityId {
        self.ecs.spawn(())
    }
}
