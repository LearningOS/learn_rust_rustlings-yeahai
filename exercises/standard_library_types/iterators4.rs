// iterators4.rs

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
<<<<<<< HEAD
=======
    (1..num+1).product()
    // (1..num+1).fold(1,|sum,v| sum*v)
>>>>>>> 90fab5c (2022/7/17/14.12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
