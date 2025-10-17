use crate::types::*;
use crate::constants::CHUNK_SIZE;

#[derive(Clone)]
pub enum Position {
    Free {
        x: PosType,
        y: PosType,
        top: bool,
    },

    Tile {
        chunk: ChunkPos,
        col: u8,
        row: u8,
        top: bool,
    },
}

#[derive(Clone, PartialEq)]
pub struct ChunkPos {
    pub x: ChunkPosType,
    pub y: ChunkPosType,
}

impl Position {
    pub fn free(x: PosType, y: PosType) -> Self {
        Self::Free {
            x,
            y,
            top: false,
        }
    }

    pub fn tile(chunk: ChunkPos, col: u8, row: u8) -> Self {
        Self::Tile {
            chunk,
            col,
            row,
            top: false,
        }
    }

    pub fn top(&mut self) -> &mut Self {
        match self {
            Self::Free { top, .. } => { *top = true; },
            Self::Tile { top, .. } => { *top = true; },
        };
        self
    }

    pub fn move_x(&mut self, amount: PosType) { 
        match self {
            Self::Free { x, .. } => { *x += amount; },
            Self::Tile { .. } => {},
        }
    }

    pub fn move_y(&mut self, amount: PosType) { 
        match self {
            Self::Free { y, .. } => { *y += amount; },
            Self::Tile { .. } => {},
        }
    }

    pub fn x(&self) -> PosType {
        match &self {
            Self::Free { x, .. } => *x,
            Self::Tile { chunk, col, .. } =>
                chunk.x as PosType * CHUNK_SIZE as PosType + *col as PosType 
        }
    }

    pub fn y(&self) -> PosType {
        match &self {
            Self::Free { y, .. } => *y,
            Self::Tile { chunk, row, .. } =>
                chunk.y as PosType * CHUNK_SIZE as PosType + *row as PosType 
        }
    }

    pub fn chunk(&self) -> ChunkPos {
        match &self {
            Self::Free { x, y, .. } => ChunkPos::new(
                (x / CHUNK_SIZE as PosType).floor() as ChunkPosType,
                (y / CHUNK_SIZE as PosType).floor() as ChunkPosType),
            Self::Tile { chunk, .. } => chunk.clone()
        }
    }

    pub fn order(&self) -> usize {
        let top = match self {
            Self::Free { top, .. } => top,
            Self::Tile { top, .. } => top,
        };

        if self.is_free() {
            if *top { 4 } else { 1 }
        } else {
            if *top { 3 } else { 0 }
        }
    }

    pub fn is_free(&self) -> bool {
        match &self {
            Self::Free { .. } => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_tile(&self) -> bool {
        match &self {
            Self::Tile { .. } => true,
            _ => false
        }
    }
}

impl ChunkPos {
    pub fn new(x: ChunkPosType, y: ChunkPosType) -> Self {
        Self{x, y}}}
