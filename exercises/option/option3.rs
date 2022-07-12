// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
<<<<<<< HEAD
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
=======
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
>>>>>>> 90fab5c (2022/7/17/14.12)
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
