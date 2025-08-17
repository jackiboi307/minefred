use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(Position::free(1.0, 1.0))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("test")
    .init(init)
    .texture("error")
;
