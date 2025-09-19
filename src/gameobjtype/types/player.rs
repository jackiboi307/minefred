use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder> {
    entity
        .add(Player::new())
        .add(Position::free(0.0, 0.0))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("player")
    .class(GameObjectClass::entity())
    .init(init)
    .texture("player")
;
