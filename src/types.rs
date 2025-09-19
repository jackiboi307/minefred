pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
pub type Font<'a> = sdl2::ttf::Font<'a, 'a>;

pub type PosType = f32;
pub type SizeType = u16;
pub type ChunkPosType = i32;
pub type GameObjectTypeId = u8;
pub type UpdateFnIdType = GameObjectTypeId;
