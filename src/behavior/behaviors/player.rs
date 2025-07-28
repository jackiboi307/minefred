use crate::behavior::base::*;
use crate::behavior::render::texture::render;
use crate::types::*;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        textures: &Textures) -> Result<(), Error> {

    ecs.insert(ecs_id, (
        Position::free(0.0, 0.0),
        TextureComponent::new(textures, "player"),
    ))?;
    Ok(())
}

fn update(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        update_data: &UpdateData) -> Result<(), Error> {

    if let Ok(mut pos) = ecs.get::<&mut Position>(ecs_id) {
        let speed =
            if update_data.is_pressed([K::LShift])
                { 0.2 } else { 0.1 };

        if update_data.is_pressed([K::D, K::Right]) { pos.move_x(speed); }
        if update_data.is_pressed([K::A, K::Left])  { pos.move_x(-speed); }
        if update_data.is_pressed([K::S, K::Down])  { pos.move_y(speed); }
        if update_data.is_pressed([K::W, K::Up])    { pos.move_y(-speed); }
    }

    Ok(())
}

#[allow(non_upper_case_globals)]
pub const PlayerBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update,
};
