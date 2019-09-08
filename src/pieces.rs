// Shapes are defined in a 4x4 Field
// Each shape has its corresponding rotation maps already configured
// The origin of this 4x4 Field is in the top left corner
//

use crate::defs::*;

#[derive(Debug,Default,Clone)]
pub struct Shape {
    pub piece_type: char,
    pub blocks: Vec<Block>,
    pub rotation: i8,
    pub x: i8,
    pub y: i8,
    defMask: Vec<Block>,
    rotMask1: Vec<Block>,
    rotMask2: Vec<Block>,
    rotMask3: Vec<Block>,
}

impl Shape{
    // IPiece Rotation Maps
    // 0         1         2        3
    //
    // 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
    // 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
    // 1 0 0 0   0 0 0 0   1 0 0 0  0 0 0 0
    // 1 0 0 0   1 1 1 1   1 0 0 0  1 1 1 1
    //
    pub fn init_IPiece(&mut self,x: i8,y: i8){
        self.piece_type='I';
        self.x = x;
        self.y = y;
        self.rotation = 0;

        self.defMask = vec![
            Block{x:self.x+0,y:self.y+0},
            Block{x:self.x+1,y:self.y+0},
            Block{x:self.x+2,y:self.y+0},
            Block{x:self.x+3,y:self.y+0}
            ];

        self.rotMask1 = vec![
            Block{x:self.x+0,y:self.y+0},
            Block{x:self.x+0,y:self.y+1},
            Block{x:self.x+0,y:self.y+2},
            Block{x:self.x+0,y:self.y+3}
            ];

        self.rotMask2 = vec![
            Block{x:self.x+0,y:self.y+0},
            Block{x:self.x+1,y:self.y+0},
            Block{x:self.x+2,y:self.y+0},
            Block{x:self.x+3,y:self.y+0}
            ];
            
        self.rotMask3 = vec![
            Block{x:self.x+0,y:self.y+0},
            Block{x:self.x+0,y:self.y+1},
            Block{x:self.x+0,y:self.y+2},
            Block{x:self.x+0,y:self.y+3}
            ];


        self.blocks = self.defMask.clone();
    }

    pub fn rotate(&mut self){
        self.rotation = (self.rotation+1)%4;
        match self.rotation {
            0 => {
                    let mut clone = Vec::new();
                    for mut block in self.defMask.clone() {
                        block.x += self.x;
                        block.y += self.y;
                        clone.push(block);
                    } 
                    self.blocks = clone.clone();
                    }
            1 => {                     
                    let mut clone = Vec::new();
                    for mut block in self.rotMask1.clone() {
                        block.x += self.x;
                        block.y += self.y;
                        clone.push(block);
                    } 
                    self.blocks = clone.clone(); 
                    }
            2 => {
                    let mut clone = Vec::new();
                    for mut block in self.rotMask2.clone() {
                        block.x += self.x;
                        block.y += self.y;
                        clone.push(block);
                    } 
                    self.blocks = clone.clone(); 
                    }
            _ => {
                    let mut clone = Vec::new();
                    for mut block in self.rotMask3.clone() {
                        block.x += self.x;
                        block.y += self.y;
                        clone.push(block);
                    } 
                    self.blocks = clone.clone();
                    }
        }
    }

    pub fn moveLeft(&mut self,mut f: Field){      
        if self.y-1 < 0 {
            return;
        } else {
            self.y = self.y-1;
            let current = self.blocks.clone();
            
            for mut block in &mut self.blocks {
                block.y -= 1;
            }

            let new = self.blocks.clone();
            for block_shape in new {
                for block_field in &mut f.placed_blocks {
                    if block_shape.x == block_field.x && block_shape.y == block_field.y {
                        self.blocks = current.clone();
                    }
                }
            }
        }

    }

    pub fn moveRight(&mut self,mut f: Field){
        if self.y+1 > f.width-4 { // width-1 is the last column -3 for the width of the shape box
            return;
        } else {
            self.y = self.y+1;
            let current = self.blocks.clone();

            for mut block in &mut self.blocks {
                block.y += 1;
            }

            let new = self.blocks.clone();
            for block_shape in new {
                for block_field in &mut f.placed_blocks {
                    if block_shape.x == block_field.x && block_shape.y == block_field.y {
                        self.blocks = current.clone();
                        return;
                    }
                }
            }
        }

    }

    pub fn moveDown(&mut self,mut f: Field) -> i8{
        let mut minX=0;

        for block in self.blocks.clone(){
            if block.x>minX{
                minX=block.x;
            }
        }
        minX -= self.x;
        if self.x+1 >  f.height-minX { // width-1 is the last column -3 for the width of the shape box
            return 1;
        } else {
            let current = self.blocks.clone();

            self.x = self.x+1;
            for mut block in &mut self.blocks {
                block.x += 1;
            }
            
            let new = self.blocks.clone();
            for block_shape in new {
                for block_field in &mut f.placed_blocks {
                    if block_shape.x == block_field.x && block_shape.y == block_field.y {
                        self.blocks = current.clone();
                        return 1;
                    }
                }
            }
        }
        return 0;

    }
}

