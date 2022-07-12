// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

fn main() {
    let data = "Rust is great!".to_string();

<<<<<<< HEAD
    get_char(data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
=======
    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
>>>>>>> 90fab5c (2022/7/17/14.12)
    data.chars().last().unwrap()
}

// Should take ownership
<<<<<<< HEAD
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();
=======
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
>>>>>>> 90fab5c (2022/7/17/14.12)

    println!("{}", data);
}
