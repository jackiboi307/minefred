use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder> {
    entity
        .add(TextureTransform::new()
            .scale(2.0))
    ;

    entity.get_mut::<&mut Position>().unwrap().top();

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("tree")
    .class(GameObjectClass::block())
    .init(init)
    .texture("tree")
;
