// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

<<<<<<< HEAD
// I AM NOT DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */ = cat;
=======

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age)/* your pattern here */ = cat;
>>>>>>> 90fab5c (2022/7/17/14.12)

    println!("{} is {} years old.", name, age);
}
