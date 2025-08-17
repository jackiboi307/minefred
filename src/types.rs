pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub type PosType = f32;
pub type SizeType = u16;
pub type ChunkPosType = i32;
pub type GameObjectTypeId = u8;
pub type UpdateFnIdType = GameObjectTypeId;
