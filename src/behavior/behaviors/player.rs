use crate::behavior::base::*;
use crate::behavior::render::rect::render;
use crate::types::*;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        _: &Textures) -> Result<(), Error> {

    ecs.insert(ecs_id, (Position::new(0.0, 0.0),))?;
    Ok(())
}

fn update(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        update_data: &UpdateData) -> Result<(), Error> {

    if let Ok(mut pos) = ecs.get::<&mut Position>(ecs_id) {
        let speed =
            if update_data.is_pressed([K::LShift])
                { 32.0 } else { 0.1 };

        if update_data.is_pressed([K::D, K::Right]) { pos.x += speed; }
        if update_data.is_pressed([K::A, K::Left])  { pos.x -= speed; }
        if update_data.is_pressed([K::S, K::Down])  { pos.y += speed; }
        if update_data.is_pressed([K::W, K::Up])    { pos.y -= speed; }
    }

    Ok(())
}

#[allow(non_upper_case_globals)]
pub const PlayerBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update,
};
