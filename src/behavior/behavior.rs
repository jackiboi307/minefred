use crate::behavior::base::*;

pub struct GameObjectBehavior {
    pub init: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId),

    pub update: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _update_data: &UpdateData),

    pub render: fn(
        &ECSWorld,
        ECSEntityId,
        &mut Canvas
    ),
}
