use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder> {
    entity
        .add(TextureTransform::new()
            .random_direction())
    ;

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("test_tile")
    .class(GameObjectClass::block())
    .init(init)
    .texture("grass")
;
