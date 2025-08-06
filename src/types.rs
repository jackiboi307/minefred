pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type PosType = f32;
pub type SizeType = u16;
pub type ChunkPosType = i32;

pub type UpdateFnIdType = u8;

#[derive(Clone, Copy)]
pub struct Rect {
    pub width:  SizeType,
    pub height: SizeType,
}

impl Rect {
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self{width, height} }}

