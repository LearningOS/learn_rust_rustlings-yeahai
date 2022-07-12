// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
<<<<<<< HEAD
    let second = ???;
=======
    let second = numbers.1;
>>>>>>> 90fab5c (2022/7/17/14.12)

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
