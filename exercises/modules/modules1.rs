// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

<<<<<<< HEAD
    fn make_sausage() {
=======
    pub fn make_sausage() {
>>>>>>> 90fab5c (2022/7/17/14.12)
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
