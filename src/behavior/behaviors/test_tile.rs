use crate::behavior::base::*;
use crate::constants::TILE_SIZE;
use crate::textures::TextureId;

use sdl2::rect;

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId) {

    ecs.insert(ecs_id, (TextureId("test",),)).unwrap();
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

    let texture = ecs.get::<&TextureId>(ecs_id).unwrap();
    let texture = textures.get(&texture).unwrap();

    canvas.copy(&texture, None, Some(
        rect::Rect::new(
            pos.x + (TILE_SIZE.width  as i32 - 1) * info.tile.as_ref().unwrap().pos.x,
            pos.y + (TILE_SIZE.height as i32 - 1) * info.tile.as_ref().unwrap().pos.y,
            TILE_SIZE.width.into(),
            TILE_SIZE.height.into(),
    ))).unwrap();
}

#[allow(non_upper_case_globals)]
pub const TestTileBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
