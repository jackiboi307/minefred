// Types

pub type PosType  = i32;
pub type SizeType = u16;

// General stuff

// Position

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position { // TODO byt namn
    pub x: PosType,
    pub y: PosType,
}

impl Position {
    pub fn new(x: PosType, y: PosType) -> Self {
        Self{x, y}
    }
}

// Position aliases
pub type Offset = Position;
pub type GridPos = Position;

// Rect

#[derive(Clone, Copy)]
pub struct Rect {
    pub width:  SizeType,
    pub height: SizeType,
}

impl Rect {
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self{width, height}
    }

    // pub fn center(&self) -> Position {
    //     Position::new(self.width/2, self.height/2)
    // }
}
