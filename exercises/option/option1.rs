// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
<<<<<<< HEAD
    print_number(13);
    print_number(99);

    let mut numbers: [Option<u16>; 5];
=======
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5]= [Some(0);5];
>>>>>>> 90fab5c (2022/7/17/14.12)
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

<<<<<<< HEAD
        numbers[iter as usize] = number_to_add;
=======
        numbers[iter as usize] = Some(number_to_add);
>>>>>>> 90fab5c (2022/7/17/14.12)
    }
}
