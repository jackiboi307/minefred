use crate::behavior::base::*;
use crate::textures::{TextureComponent, copy_texture};

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        textures: &Textures) -> Result<(), Error> {

    let texture = TextureComponent::new(&textures, "dirt");
    ecs.insert(ecs_id, (texture,))?;
    Ok(())
}

pub fn render(
        ecs: &ECSWorld,
        ecs_id: ECSEntityId,
        info: &RenderInfo,
        textures: &Textures,
        canvas: &mut Canvas) -> Result<(), Error> {

    let texture = ecs.get::<&TextureComponent>(ecs_id)?;

    copy_texture(canvas, &textures, &texture, info.rect)?;

    Ok(())
}

#[allow(non_upper_case_globals)]
pub const TestTileBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
