// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

<<<<<<< HEAD
    let nice_slice = ???
=======
    let nice_slice = &a[1..4];
>>>>>>> 90fab5c (2022/7/17/14.12)

    assert_eq!([2, 3, 4], nice_slice)
}
