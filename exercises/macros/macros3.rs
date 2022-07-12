// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

<<<<<<< HEAD
// I AM NOT DONE

=======
#[warn(unused_macros)]
#[macro_use]
>>>>>>> 90fab5c (2022/7/17/14.12)
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
