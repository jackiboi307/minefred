use crate::gameobjtype::base::*;
use crate::types::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(Position::free(0.0, 0.0))
        .add(TextureComponent::new("player"))
    ;

    Ok(entity)
}

fn update(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        update_data: &UpdateData) -> Result<(), Error> {

    if let Ok(mut pos) = ecs.get::<&mut Position>(ecs_id) {
        let speed =
            if update_data.events.key("RUN")
                { 0.2 } else { 0.1 };

        if update_data.events.key("MOVE_RIGHT") { pos.move_x(speed); }
        if update_data.events.key("MOVE_LEFT")  { pos.move_x(-speed); }
        if update_data.events.key("MOVE_DOWN")  { pos.move_y(speed); }
        if update_data.events.key("MOVE_UP")    { pos.move_y(-speed); }
    }

    Ok(())
}

pub const TYPE: GameObjectType = GameObjectType{
    key: "player",
    init: Some(init),
    update: Some(update),
};
