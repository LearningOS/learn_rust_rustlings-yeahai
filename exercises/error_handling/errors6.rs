// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError)
}

impl ParsePosNonzeroError {
    // TODO: add another error conversion function here.
<<<<<<< HEAD
=======
    fn from_creation(e:CreationError) -> ParsePosNonzeroError {
        match e {
            CreationError::Negative =>ParsePosNonzeroError::Creation(CreationError::Negative),
            CreationError::Zero => ParsePosNonzeroError::Creation(CreationError::Zero),
        }
    }
    fn from_parse(e:ParseIntError) -> ParsePosNonzeroError{
        match e {
            e => ParsePosNonzeroError::ParseInt(e)
        }
    }

>>>>>>> 90fab5c (2022/7/17/14.12)
}

fn parse_pos_nonzero(s: &str)
    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
<<<<<<< HEAD
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x)
        .map_err(ParsePosNonzeroError::from_creation)
=======
    let x  = s.parse::<i64>()
        .map_err(ParsePosNonzeroError::from_parse)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)

    // let x = s.parse::<i64>();
    // let x = match x {
    //     Ok(x) => x,
    //     Err(e) => return Err(ParsePosNonzeroError::ParseInt(e)),
    // };
>>>>>>> 90fab5c (2022/7/17/14.12)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
