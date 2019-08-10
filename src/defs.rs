// Structs for data structures

use crate::shapes::*;

#[derive(Debug)]
pub struct Field {
    pub height: i8,
    pub width: i8,
    pub shapes: Vec<T>,
}

#[derive(Debug)]
pub struct Rectangle {
    pub corner: Point,
    pub height: i8,
    pub length: i8,
}

#[derive(Debug)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}
