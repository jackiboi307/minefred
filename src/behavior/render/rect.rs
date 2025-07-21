use crate::behavior::base::*;
use crate::types::{Position, Rect};
use crate::TILE_SIZE;

use sdl2::rect;
use sdl2::pixels::Color;

pub fn render(
        ecs: &ECSWorld,
        ecs_id: ECSEntityId,
        info: &RenderInfo,
        canvas: &mut Canvas) {

    let pos = ecs.get::<&Position>(ecs_id).unwrap();
    let pos = calc_pos(Position::new(pos.x, pos.y), TILE_SIZE, info.offset);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(rect::Rect::new(
        pos.x,
        pos.y,
        TILE_SIZE.width.into(),
        TILE_SIZE.height.into()
    )).unwrap();
}
