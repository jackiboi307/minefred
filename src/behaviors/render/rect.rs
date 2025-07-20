use crate::behaviors::base::*;

use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub fn render(
        ecs: &ECSWorld,
        ecs_id: ECSEntityId,
        canvas: &mut Canvas) {

    let pos = ecs.get::<&Position>(ecs_id).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(pos.x, pos.y, 100, 100)).unwrap();
}
