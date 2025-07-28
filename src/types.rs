use crate::constants::*;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type PosType = f32;
pub type SizeType = u16;
pub type ChunkPosType = i32;

pub enum PosKind {
    Free{x: PosType, y: PosType},
    Tile{chunk: ChunkPos, col: u8, row: u8},
}

// TODO PosKind -> Position

pub struct Position {
    pub pos: PosKind,
}

#[derive(Clone, PartialEq)]
pub struct ChunkPos {
    pub x: ChunkPosType,
    pub y: ChunkPosType,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub width:  SizeType,
    pub height: SizeType,
}

impl Position {
    pub fn free(x: PosType, y: PosType) -> Self {
        Self{pos: PosKind::Free{x, y}} }

    pub fn tile(chunk: ChunkPos, col: u8, row: u8) -> Self {
        Self{pos: PosKind::Tile{chunk, col, row}} }

    pub fn move_x(&mut self, amount: PosType) { 
        match &mut self.pos {
            PosKind::Free{ x, .. } => { *x += amount; },
            PosKind::Tile{ .. } => {},
        }
    }

    pub fn move_y(&mut self, amount: PosType) { 
        match &mut self.pos {
            PosKind::Free{ y, .. } => { *y += amount; },
            PosKind::Tile{ .. } => {},
        }
    }

    pub fn x(&self) -> PosType {
        match &self.pos {
            PosKind::Free{ x, .. } => *x,
            PosKind::Tile{ chunk, col, .. } =>
                chunk.x as PosType * CHUNK_SIZE as PosType + *col as PosType 
        }
    }

    pub fn y(&self) -> PosType {
        match &self.pos {
            PosKind::Free{ y, .. } => *y,
            PosKind::Tile{ chunk, row, .. } =>
                chunk.y as PosType * CHUNK_SIZE as PosType + *row as PosType 
        }
    }

    pub fn chunk(&self) -> ChunkPos {
        match &self.pos {
            PosKind::Free{ x, y } => ChunkPos::new(
                (x / CHUNK_SIZE as PosType).floor() as ChunkPosType,
                (y / CHUNK_SIZE as PosType).floor() as ChunkPosType),
            PosKind::Tile{ chunk, .. } => chunk.clone()
        }
    }

    // pub fn is_free(&self) -> bool {
    //     match &self.pos {
    //         PosKind::Free{ .. } => true,
    //         _ => false
    //     }
    // }

    // pub fn is_tile(&self) -> bool {
    //     match &self.pos {
    //         PosKind::Tile{ .. } => true,
    //         _ => false
    //     }
    // }
}

impl ChunkPos {
    pub fn new(x: ChunkPosType, y: ChunkPosType) -> Self {
        Self{x, y} }}

impl Rect {
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self{width, height} }}

