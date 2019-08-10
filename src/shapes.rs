// Shapes are defined in a 4x4 Field
// Each shape has its corresponding rotation maps already configured
// The origin of this 4x4 Field is in the bottom left corner
//
// IPiece Rotation Maps
// 0         1         2        3
//
// 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
// 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
// 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
// 1 0 0 0   1 1 1 1   1 0 0 0  1 1 1 1
//
use crate::defs::*;

#[derive(Debug)]
pub struct IPiece {
    pub points: Vec<Point>,
    pub rotation: i8,
    pub x: i8,
    pub y: i8,
    pub field: Field,
}

impl IPiece {
    fn rotate_cw(&self) {
        self.rotation = (self.rotation+1) % 4;
        self.points.clear();
        match self.rotation {
            0 => {
                self.points.push(Point{x:0,y:0});
                self.points.push(Point{x:0,y:1});
                self.points.push(Point{x:0,y:2});
                self.points.push(Point{x:0,y:3});
            },

            1 => {
                self.points.push(Point{x:0,y:3}); 
                self.points.push(Point{x:1,y:3}); 
                self.points.push(Point{x:2,y:3});
                self.points.push(Point{x:3,y:3});
            },
            2 => {
                self.points.push(Point{x:0,y:0});
                self.points.push(Point{x:0,y:1});
                self.points.push(Point{x:0,y:2});
                self.points.push(Point{x:0,y:3});
            }
            _ => {
                self.points.push(Point{x:0,y:3}); 
                self.points.push(Point{x:1,y:3}); 
                self.points.push(Point{x:2,y:3});
                self.points.push(Point{x:3,y:3});
            }
        };
    }

    fn rotate_ccw(&self) {
        self.rotation = (self.rotation-1)%4;
        self.points.clear();
        match self.rotation {
            0 => {
                self.points.push(Point{x:0,y:0});
                self.points.push(Point{x:0,y:1});
                self.points.push(Point{x:0,y:2});
                self.points.push(Point{x:0,y:3});
            },

            1 => {
                self.points.push(Point{x:0,y:3}); 
                self.points.push(Point{x:1,y:3}); 
                self.points.push(Point{x:2,y:3});
                self.points.push(Point{x:3,y:3});
            },
            2 => {
                self.points.push(Point{x:0,y:0});
                self.points.push(Point{x:0,y:1});
                self.points.push(Point{x:0,y:2});
                self.points.push(Point{x:0,y:3});
            }
            _ => {
                self.points.push(Point{x:0,y:3}); 
                self.points.push(Point{x:1,y:3}); 
                self.points.push(Point{x:2,y:3});
                self.points.push(Point{x:3,y:3});
            }
        };
    }

     fn left(&self) {
        let current = self.x;
        self.x = self.x-1;
        if self.collides_with_borders(self.field) {
            self.x = current;
        }
    }

    fn right(&self) {
        let current = self.x;
        self.x = self.x+1;
        if self.collides_with_borders(self.field) {
            self.x = self.x+1;
        }
    }

    fn down(&self) {
        let current = self.y;
        self.y = self.y+1;
        if self.collides_with_borders(self.field) {
            self.y = self.y+1;
        }
    }

    fn collides_with_borders(&self, f: Field) -> bool {
        if self.x < 1 || self.x > f.width-4 || self.y > f.height-4 {
            return true;
        } else {
            return false;
        }
    }
}

impl Default for IPiece {
    fn default() -> IPiece {
        IPiece {
            x: -1,
            y: -1,
            points: vec!{Point{x:0,y:0},Point{x:0,y:1},Point{x:0,y:2},Point{x:0,y:3}},
            rotation: 0,
            field: Field{height: 30, width: 20, shapes: Vec::new()}
        }
    }
}