use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder> {
    entity
        .add(Position::free(1.0, 1.0))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("test")
    .class(GameObjectClass::entity())
    .init(init)
    .texture("error")
;
