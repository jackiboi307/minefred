use crate::gameobjtype::base::*;
use crate::types::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(Player::new())
        .add(Position::free(0.0, 0.0))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("player")
    .init(init)
    .texture("player")
;
