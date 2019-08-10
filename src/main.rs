use crate::defs::*;
use crate::shapes::*;

mod defs;
mod shapes;

fn main() {
    //create some rectangles and add them to the field
    let field = Field{height: 30, width: 15, shapes: Vec::new()};

    let shape = IPiece{x:2,y:10,field: field, Default::default()};



    // print the field
    print_field(field);
}

/**
 * Function to print the current playing field
 */

fn print_field(field: Field) {
    // list of points
    let mut used_points = Vec::new();


    



    // Print upper border
    println!("-{:-<amount$}-", "", amount = field.width as usize);

    // find which lines are affected by points
    for line in 1..field.height + 1 {
        let mut used = false;
        for point in &used_points {
            if point.x == line {
                used = true;
                break;
            }
        }

        if !used {
            println!("|{: <amount$}|", "", amount = field.width as usize);
        } else {
            let mut line_index = vec![false; field.width as usize];
            for point in &used_points {
                if point.x == line {
                    line_index[point.y as usize] = true;
                }
            }
            print!("|");
            for b in line_index {
                if b {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
    }

    // Print lower border
    println!("-{:-<amount$}-", "", amount = field.width as usize);
}
