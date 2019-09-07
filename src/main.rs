use crossterm::*;
use std::io;

use crate::defs::*;
use crate::pieces::*;
use std::{thread,time};

mod defs;
mod pieces;

fn clear_all_lines() -> io::Result<()> {
    let terminal = terminal();
    
    // Clear all lines in terminal;
    terminal.clear(ClearType::All)?;

    Ok(())
}

fn update_screen(f: &Field) {
    clear_all_lines();
    print_field(f);
}

fn process_input(key_event: InputEvent) -> char {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    'q' => { return 'q'; }

                    'a' => { return 'l'; }

                    'd' => { return 'r'; }

                    's' => { return 'd'; }

                    _ => {}
                }
                  _ => {},
            }
        }
        _ => {},
    }
    return ' ';
}

fn main() {
    // Create an empty field
    let mut field = Field {
        width: 10,
        height: 15,
        placed_blocks: Vec::new(),
        curr_shape: Shape::default(),
    };

    // add the current shape
    field.curr_shape.init_IPiece(1, 3);

    let input = input();
    let mut sync_stdin = input.read_async();
    clear_all_lines();

    while true { 
        let event = sync_stdin.next();

        let mut result = ' ';
        if let Some(key_event) = event {
            result = process_input(key_event);
        }

        match result {
            'l' => {field.curr_shape.moveLeft(field.clone());print!("left")}
            'r' => {field.curr_shape.moveRight(field.clone());}
            'd' => {field.curr_shape.moveDown(field.clone());}
            'q' => {break;}
            _ => {}
        }
        thread::sleep(time::Duration::from_secs(1));

        update_screen(&field);        
        field.curr_shape.moveDown(field.clone());
        
    }

    // save points of the shape to the field

    // loop
}

/**
 * Function to print the current playing field
 */
fn print_field(field: &Field) {
    // list of points
    let mut used_points: Vec<Block> = Vec::new();
    for block in field.curr_shape.blocks.clone() {
        used_points.push(block)
    }
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
