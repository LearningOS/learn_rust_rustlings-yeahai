// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

<<<<<<< HEAD
// I AM NOT DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
=======

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
>>>>>>> 90fab5c (2022/7/17/14.12)
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
