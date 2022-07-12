// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

#[cfg(test)]
mod tests {
    use super::*;

<<<<<<< HEAD
=======
    macro_rules! my_macro {
        ($val:expr) => {
            String::from("Hello ") + $val
        };
    }


>>>>>>> 90fab5c (2022/7/17/14.12)
    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
