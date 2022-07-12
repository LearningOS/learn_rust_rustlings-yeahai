// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

<<<<<<< HEAD
// I AM NOT DONE

fn main() {
    let mut shopping_list: Vec<?> = Vec::new();
=======

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
>>>>>>> 90fab5c (2022/7/17/14.12)
    shopping_list.push("milk");
}
