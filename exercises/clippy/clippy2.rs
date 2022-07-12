// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

fn main() {
    let mut res = 42;
    let option = Some(12);
<<<<<<< HEAD
    for x in option {
=======
    if let Some(x) = option {
>>>>>>> 90fab5c (2022/7/17/14.12)
        res += x;
    }
    println!("{}", res);
}
