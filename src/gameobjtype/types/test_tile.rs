use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(TextureTransform::new()
            .random_direction())
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("test_tile")
    .init(init)
    .texture("grass")
;
