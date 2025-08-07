use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(
            Texture::new("grass")
            .random_direction())
    ;

    Ok(entity)
}

pub const TYPE: GameObjectType = GameObjectType{
    key: "test_tile",
    init: Some(init),
    update: None,
};
