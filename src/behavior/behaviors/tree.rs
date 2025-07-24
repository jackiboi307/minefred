use crate::behavior::base::*;
use crate::behavior::render::texture::render;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        textures: &Textures) -> Result<(), Error> {

    let texture =
        TextureComponent::new(&textures, "tree")
        .set_scale(2.0);
        // .random_direction();
    ecs.insert(ecs_id, (texture,))?;
    Ok(())
}

#[allow(non_upper_case_globals)]
pub const TreeBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
