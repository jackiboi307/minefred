use crate::behavior::base::*;
use crate::behavior::render::texture::render;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        textures: &Textures) -> Result<(), Error> {

    ecs.insert(ecs_id, (
        Position::new(0.0, 0.0),
        TextureComponent::new(&textures, "error"),
    ))?;
    Ok(())
}

#[allow(non_upper_case_globals)]
pub const TestBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
