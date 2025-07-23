// Types

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type PosType = f32;
pub type SizeType = u16;
pub type GridPosType  = i32;

// General stuff

// Position

pub struct Position { // TODO byt namn
    pub x: PosType,
    pub y: PosType,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct GridPos {
    pub x: GridPosType,
    pub y: GridPosType,
}

// Rect

#[derive(Clone, Copy)]
pub struct Rect {
    pub width:  SizeType,
    pub height: SizeType,
}

// Impl

impl Position {
    pub fn new(x: PosType, y: PosType) -> Self {
        Self{x, y} }}

impl GridPos {
    pub fn new(x: GridPosType, y: GridPosType) -> Self {
        Self{x, y} }}

impl Rect {
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self{width, height} } }
