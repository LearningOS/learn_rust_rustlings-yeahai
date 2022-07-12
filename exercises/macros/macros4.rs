// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
<<<<<<< HEAD
    }
=======
    };
>>>>>>> 90fab5c (2022/7/17/14.12)
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
