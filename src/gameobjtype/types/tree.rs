use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(TextureTransform::new()
            .scale(2.0))
    ;

    entity.get_mut::<&mut Position>().ok_or("no position specified")?.top();

    Ok(entity)
}

pub const TYPE: GameObjectTypeBuilder = 
    *GameObjectTypeBuilder::new("tree")
    .init(init)
    .texture("tree")
;
