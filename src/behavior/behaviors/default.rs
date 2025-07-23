use crate::behavior::base::*;

fn default_init(
    _ecs: &mut ECSWorld,
    _ecs_id: ECSEntityId,
    _textures: &Textures) -> Result<(), Error> {

    Ok(())
}

fn default_update(
    _ecs: &mut ECSWorld,
    _ecs_id: ECSEntityId,
    _update_data: &UpdateData) -> Result<(), Error> {

    Ok(())
}

fn default_render(
    _ecs: &ECSWorld,
    _ecs_id: ECSEntityId,
    _render_info: &RenderInfo,
    _textures: &Textures,
    _canvas: &mut Canvas) -> Result<(), Error> {

    Ok(())
}

#[allow(non_upper_case_globals)]
pub const DefaultBehavior: GameObjectBehavior = GameObjectBehavior{
    init: default_init,
    update: default_update,
    render: default_render,
};
