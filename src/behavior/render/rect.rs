use crate::behavior::base::*;

use sdl2::pixels::Color;

pub fn render(
        _: &ECSWorld,
        _: ECSEntityId,
        info: &RenderInfo,
        _: &Textures,
        canvas: &mut Canvas) -> Result<(), Error> {

    // let pos = ecs.get::<&Position>(ecs_id)?;
    // let pos = calc_pos(Position::new(pos.x, pos.y), TILE_SIZE, info.offset);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(info.rect)?;/*rect::Rect::new(
        pos.x,
        pos.y,
        TILE_SIZE.width.into(),
        TILE_SIZE.height.into()
    ))?;*/

    Ok(())
}
