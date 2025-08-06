use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(Position::free(1.0, 1.0))
        .add(TextureComponent::new("error"))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectType = GameObjectType{
    key: "test",
    init: Some(init),
    update: None,
};
