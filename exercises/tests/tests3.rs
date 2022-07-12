// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
<<<<<<< HEAD
        assert!();
=======
        assert!(is_even(4));
>>>>>>> 90fab5c (2022/7/17/14.12)
    }

    #[test]
    fn is_false_when_odd() {
<<<<<<< HEAD
        assert!();
=======
        assert!(!is_even(5));
>>>>>>> 90fab5c (2022/7/17/14.12)
    }
}
