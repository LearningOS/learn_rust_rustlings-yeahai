// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

<<<<<<< HEAD
// I AM NOT DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

=======

use std::num::ParseIntError;

fn main() ->Result<(),ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // let cost = total_cost(pretend_user_input).unwrap();
>>>>>>> 90fab5c (2022/7/17/14.12)
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
<<<<<<< HEAD
=======
    Ok(())
>>>>>>> 90fab5c (2022/7/17/14.12)
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
