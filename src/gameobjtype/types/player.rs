use crate::gameobjtype::base::*;
use crate::types::*;

fn init<'a>(entity: &'a mut EntityBuilder) -> Result<&'a mut EntityBuilder, Error> {
    entity
        .add(Player::new())
        .add(Position::free(0.0, 0.0))
        .add(Texture::new("player"))
    ;

    Ok(entity)
}

pub const TYPE: GameObjectType = *GameObjectType::new("player").init(init);
