// Structs for data structures
use crate::pieces::Shape;

#[derive(Debug,Clone)]
pub struct Field {
    pub height: i8,
    pub width: i8,
    pub placed_blocks: Vec<Block>,
    pub curr_shape: Shape,
}

#[derive(Debug,Copy,Clone)]
pub struct Block {
    pub x: i8,
    pub y: i8,
}
