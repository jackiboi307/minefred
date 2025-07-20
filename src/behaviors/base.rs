// Almost everything required to define new behaviors.

pub use crate::update::UpdateData;
pub use crate::components::*;

pub use hecs::World as ECSWorld;
pub use hecs::Entity as ECSEntityId;

pub use sdl2::keyboard::Scancode as K;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub trait GameObjectBehavior {
    fn init(
        &self,
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId) {}

    fn update(
        &self,
        _ecs: &mut ECSWorld,
        _ecs_id: ECSEntityId,
        _update_data: &UpdateData) {}

    fn render(&self,
        _ecs: &ECSWorld,
        _ecs_id: ECSEntityId,
        _canvas: &mut Canvas) {}
}
