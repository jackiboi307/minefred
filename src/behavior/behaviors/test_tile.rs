use crate::behavior::base::*;
use crate::constants::TILE_SIZE;
use crate::textures::{TextureComponent, copy_texture};

use sdl2::rect;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId,
        textures: &Textures) {

    let texture = TextureComponent::new(&textures, "dirt");
    ecs.insert(ecs_id, (texture,)).unwrap();
}

pub fn render(
        ecs: &ECSWorld,
        ecs_id: ECSEntityId,
        info: &RenderInfo,
        textures: &Textures,
        canvas: &mut Canvas) {

    let pos = calc_pos(
        info.tile.as_ref().unwrap().pos,
        TILE_SIZE,
        info.offset);

    let texture = ecs.get::<&TextureComponent>(ecs_id).unwrap();

    copy_texture(canvas, &textures, &texture, 
        rect::Rect::new(
            pos.x + (TILE_SIZE.width  as i32 - 1) * info.tile.as_ref().unwrap().pos.x,
            pos.y + (TILE_SIZE.height as i32 - 1) * info.tile.as_ref().unwrap().pos.y,
            TILE_SIZE.width.into(),
            TILE_SIZE.height.into(),
    ));
}

#[allow(non_upper_case_globals)]
pub const TestTileBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
