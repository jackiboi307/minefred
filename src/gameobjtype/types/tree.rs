use crate::gameobjtype::base::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(
            Texture::new("tree")
            .set_scale(2.0))
    ;

    entity.get_mut::<&mut Position>().ok_or("no position specified")?.top();

    Ok(entity)
}

pub const TYPE: GameObjectType = GameObjectType{
    key: "tree",
    init: Some(init),
    update: None,
};
