use crate::constants::CHUNK_SIZE;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type PosType = f32;
pub type SizeType = u16;
// TODO ChunkPosType: byt till i16
pub type ChunkPosType = i32;
pub type TilePosType = i32;

 // TODO byt namn
pub struct Position {
    pub x: PosType,
    pub y: PosType,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct ChunkPos {
    pub x: ChunkPosType,
    pub y: ChunkPosType,
}

pub struct TilePos {
    pub chunk: ChunkPos,
    pub chunk_x: usize,
    pub chunk_y: usize,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub width:  SizeType,
    pub height: SizeType,
}

impl Position {
    pub fn new(x: PosType, y: PosType) -> Self {
        Self{x, y} }}

impl ChunkPos {
    pub fn new(x: ChunkPosType, y: TilePosType) -> Self {
        Self{x, y} }}

impl TilePos {
    pub fn new(chunk: ChunkPos, chunk_x: usize, chunk_y: usize) -> Self {
        Self{chunk, chunk_x, chunk_y} }

    pub fn x(&self) -> TilePosType {
        self.chunk.x as TilePosType * CHUNK_SIZE as TilePosType
        + self.chunk_x as TilePosType
    }

    pub fn y(&self) -> TilePosType {
        self.chunk.y as TilePosType * CHUNK_SIZE as TilePosType
        + self.chunk_y as TilePosType
    }
}

impl Rect {
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self{width, height} } }

