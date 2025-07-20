use crate::behaviors::base::*;
use crate::behaviors::render::rect::render;

pub struct TestBehavior{}

impl GameObjectBehavior for TestBehavior {
    fn init(
            &self, 
            ecs: &mut ECSWorld, 
            ecs_id: ECSEntityId) {

        ecs.insert(ecs_id, (Position{x: 0, y: 0},)).unwrap();
    }

    fn update(
            &self, 
            ecs: &mut ECSWorld, 
            ecs_id: ECSEntityId,
            update_data: &UpdateData) {

        if let Ok(mut pos) = ecs.get::<&mut Position>(ecs_id) {
            let speed =
                if update_data.is_pressed([K::LShift])
                    { 10 } else { 5 };

            if update_data.is_pressed([K::D, K::Right]) { pos.x += speed; }
            if update_data.is_pressed([K::A, K::Left])  { pos.x -= speed; }
            if update_data.is_pressed([K::S, K::Down])  { pos.y += speed; }
            if update_data.is_pressed([K::W, K::Up])    { pos.y -= speed; }
        }
    }

    fn render(
            &self, 
            ecs: &ECSWorld, 
            ecs_id: ECSEntityId,
            canvas: &mut Canvas) {

        render(ecs, ecs_id, canvas);
    }
}
