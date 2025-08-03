use crate::gameobjtype::base::*;
use crate::event::Events;
use crate::types::*;
use crate::textures::Textures;

use sdl2::rect;

#[derive(Copy, Clone)]
pub struct GameObjectBehavior {
    pub init: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _textures: &Textures) -> Result<(), Error>,

    pub update: fn(
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _update_data: &UpdateData) -> Result<(), Error>,

    pub render: fn(
        _ecs: &ECSWorld,
        _ecs_id: ECSEntityId,
        _render_info: &RenderInfo,
        textures: &Textures,
        _canvas: &mut Canvas) -> Result<(), Error>,
}

pub struct UpdateData {
    pub events: Events,
    // TODO delta time
}

pub struct RenderInfo {
    pub rect: rect::Rect,

    #[allow(dead_code)]
    pub screen: Rect,
}
