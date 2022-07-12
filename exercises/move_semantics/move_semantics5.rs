// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

fn main() {
    let mut x = 100;
    let y = &mut x;
<<<<<<< HEAD
    let z = &mut x;
    *y += 100;
=======
    *y += 100;
    let z = &mut x;
>>>>>>> 90fab5c (2022/7/17/14.12)
    *z += 1000;
    assert_eq!(x, 1200);
}
