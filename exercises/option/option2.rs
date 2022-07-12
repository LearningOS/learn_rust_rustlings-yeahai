// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
<<<<<<< HEAD
    word = optional_word {
=======
    if let Some(word) =  optional_word {
>>>>>>> 90fab5c (2022/7/17/14.12)
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
<<<<<<< HEAD
    integer = optional_integers_vec.pop() {
=======
    while let Some(Some(integer)) = optional_integers_vec.pop() {
>>>>>>> 90fab5c (2022/7/17/14.12)
        println!("current value: {}", integer);
    }
}
