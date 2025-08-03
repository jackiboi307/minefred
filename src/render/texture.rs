use crate::gameobjtype::base::*;
use crate::textures::{copy_texture, TextureComponent};

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
