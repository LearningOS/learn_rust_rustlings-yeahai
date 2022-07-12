// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

mod delicious_snacks {

    // TODO: Fix these use statements
<<<<<<< HEAD
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???
=======
    pub use self::fruits::PEAR as fruit ;
    pub use self::veggies::CUCUMBER as veggie;
>>>>>>> 90fab5c (2022/7/17/14.12)

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
<<<<<<< HEAD
        delicious_snacks::veggie
=======
        delicious_snacks::veggie,
>>>>>>> 90fab5c (2022/7/17/14.12)
    );
}
