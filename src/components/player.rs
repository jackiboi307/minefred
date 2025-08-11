use hecs::Entity as EntityId;

pub struct Player {
    pub selected: Option<EntityId>,
}

impl Player {
    pub fn new() -> Self {
        Self { 
            selected: None,
        }
    }
}
