use crate::event::ActionState;
use hecs::Entity as EntityId;

pub struct Player {
    pub selected: Option<EntityId>,
    pub action_state: ActionState,
}

impl Player {
    pub fn new() -> Self {
        Self { 
            selected: None,
            action_state: ActionState::new(),
        }
    }
}
