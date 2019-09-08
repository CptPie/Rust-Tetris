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

                    'r' => { return 't'; }

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

    // Setup Terminal
    let screen = RawScreen::into_raw_mode();
    let input = input();
    let mut sync_stdin = input.read_async();
    clear_all_lines();

    // Game loop
    while true { 
        
        // Handle button presses
        let event = sync_stdin.next();
        let mut result = ' ';
        if let Some(key_event) = event {
            result = process_input(key_event);
        }

        let mut returncode = 0;
        match result {
            'l' => {field.curr_shape.moveLeft(field.clone()); update_screen(&field); }
            'r' => {field.curr_shape.moveRight(field.clone()); update_screen(&field); }
            'd' => { returncode += field.curr_shape.moveDown(field.clone()); update_screen(&field); }
            't' => {field.curr_shape.rotate(); update_screen(&field); }
            'q' => {break;}
            _ => {}
        }

        // update after user action
        update_screen(&field);

        // wait for auto drop        
        thread::sleep(time::Duration::from_secs(1));

        // autodrop 
        returncode += field.curr_shape.moveDown(field.clone());
        update_screen(&field);
        if returncode>0 {
            field.placed_blocks.append(&mut field.curr_shape.blocks.clone());
            field.curr_shape.init_IPiece(1, 3);
        }
        
        
    }

    // save points of the shape to the field

    // loop
}

/**
 * Function to print the current playing field
 */
fn print_field(field: &Field) -> io::Result<()> {
    // list of points
    let mut used_points: Vec<Block> = Vec::new();
    for block in field.curr_shape.blocks.clone() {
        used_points.push(block)
    }
    for block in field.placed_blocks.clone() {
        used_points.push(block)
    }
    // Print upper border
    let cursor = cursor();
    cursor.hide()?;
    cursor.goto(0, 0);
    print!("-{:-<amount$}-", "", amount = field.width as usize);

    let mut lastLine = 0;
    // find which lines are affected by points
    for line in 1..field.height + 1 {
        let mut used = false;
        for point in &used_points {
            if point.x == line {
                used = true;
                break;
            }
        }
        cursor.goto(0, line as u16);
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
        lastLine = line;
    }
    cursor.goto(0, (lastLine+1) as u16);
    // Print lower border
    println!("-{:-<amount$}-", "", amount = field.width as usize);

    //stdout.flush()?;
    Ok(())
}
