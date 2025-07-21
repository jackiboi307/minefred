// Almost everything required to define new behaviors.

#![allow(dead_code)]
#![allow(unused_imports)]

pub use crate::behavior::*;
pub use crate::components::*;
pub use crate::behavior::behaviors::DefaultBehavior;
pub use crate::textures::Textures;

pub use hecs::World as ECSWorld;
pub use hecs::Entity as ECSEntityId;

pub use sdl2::keyboard::Scancode as K;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

use crate::types::*;

pub fn calc_pos(pos: Position, size: Rect, offset: Offset) -> Position {
    Position::new(
        pos.x - offset.x - size.width  as PosType / 2,
        pos.y - offset.y - size.height as PosType / 2,
    )
}
