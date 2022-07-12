// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
<<<<<<< HEAD
        Ok(PositiveNonzeroInteger(value as u64))
=======
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            _=>Ok(PositiveNonzeroInteger(value as u64))
        }
>>>>>>> 90fab5c (2022/7/17/14.12)
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
