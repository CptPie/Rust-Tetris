use std::collections::LinkedList;


#[derive(Debug)]
struct Field {
    height: i32,
    width: i32,
    shapes: LinkedList<Rectangle>,
}

#[derive(Debug)]
struct Rectangle {
    corner: Point,
    height: i32,
    length: i32,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let rect1 = Rectangle{ corner:Point {x:3,y:5},height:3,length:2};

    let rect2 = Rectangle{ corner:Point {x:10,y:15},height:2,length:12};
    let rect3 = Rectangle{ corner:Point {x:20,y:15},height:8,length:4};
    let mut list = LinkedList::new();

    list.push_back(rect1);
    list.push_back(rect2);
    list.push_back(rect3);
    let field=Field{height:30,width:35,shapes:list};

    printField(field);

}

fn printField(field: Field) {
    let mut used_points = LinkedList::new();

    for shape in field.shapes{
        println!("{:?}",shape);
        for i in shape.corner.x..shape.corner.x+shape.height{
            for j in shape.corner.y..shape.corner.y+shape.length{
                used_points.push_back(Point{x:i,y:j});
            }
        }
    }

    // Print upper border
    println!("-{:-<amount$}-","",amount=field.width as usize);

    for line in 1..field.height+1 {
        let mut used = false;
        for point in &used_points {
            if point.x==line{
                used = true;
                break;
            }
        }

        if !used {
            println!("|{: <amount$}|","",amount=field.width as usize);
        } else {
            let mut lineIndex = vec![false;field.width as usize];
            for point in &used_points{
                if point.x==line{
                    lineIndex[point.y as usize]=true;
                }
            }
            print!("|");
            for b in lineIndex{
                if b{
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
    }

    // Print lower border
    println!("-{:-<amount$}-","",amount=field.width as usize);
}
