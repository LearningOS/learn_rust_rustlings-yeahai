// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

<<<<<<< HEAD
// I AM NOT DONE

use std::f32;

fn main() {
    let pi = 3.14f32;
=======
use std::f32;
//
fn main() {
    let pi = std::f32::consts::PI;
>>>>>>> 90fab5c (2022/7/17/14.12)
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
