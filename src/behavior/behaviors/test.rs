use crate::behavior::base::*;
use crate::behavior::render::rect::render;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        _: &Textures) -> Result<(), Error> {

    ecs.insert(ecs_id, (Position::new(0.0, 0.0),))?;
    Ok(())
}

#[allow(non_upper_case_globals)]
pub const TestBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
