use crate::behavior::base::*;
use crate::{MIN, MAX, TILE_SIZE};
use crate::random;

use sdl2::rect;
use sdl2::pixels::Color as SDLColor;

const RED:   SDLColor = SDLColor::RGB(255, 0, 0);
const GREEN: SDLColor = SDLColor::RGB(0, 255, 0);
const BLUE:  SDLColor = SDLColor::RGB(0, 0, 255);

struct Color {
    color: SDLColor,
}

fn init(
        ecs: &mut ECSWorld, 
        ecs_id: ECSEntityId) {

    ecs.insert(ecs_id, (Color{color: (
        match random::int(0..=2) {
            MIN..=0       => RED,
                  1       => GREEN,
                  2..=MAX => BLUE,
        }
    )},)).unwrap();
}

pub fn render(
        ecs: &ECSWorld,
        ecs_id: ECSEntityId,
        info: &RenderInfo,
        canvas: &mut Canvas) {

    let pos = calc_pos(
        info.tile.as_ref().unwrap().pos,
        TILE_SIZE,
        info.offset);

    let color = ecs.get::<&Color>(ecs_id).unwrap();

    canvas.set_draw_color(color.color);
    canvas.fill_rect(rect::Rect::new(
        pos.x + (TILE_SIZE.width  as i32 - 1) * info.tile.as_ref().unwrap().pos.x,
        pos.y + (TILE_SIZE.height as i32 - 1) * info.tile.as_ref().unwrap().pos.y,
        TILE_SIZE.width.into(),
        TILE_SIZE.height.into(),
    )).unwrap();
}

#[allow(non_upper_case_globals)]
pub const TestTileBehavior: GameObjectBehavior = GameObjectBehavior{
    init,
    render,
    update: DefaultBehavior.update,
};
