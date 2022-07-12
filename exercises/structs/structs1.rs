// structs1.rs
// Address all the TODOs to make the tests pass!

<<<<<<< HEAD
// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */);
=======

struct ColorClassicStruct {
    // TODO: Something goes here
    name:String,
    hex:String,
}

struct ColorTupleStruct(String,String/* TODO: Something goes here */);
>>>>>>> 90fab5c (2022/7/17/14.12)

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
<<<<<<< HEAD
        // let green =
=======
         let green = ColorClassicStruct{
             name : String::from("green"),
             hex : String::from("#00FF00"),
         };
>>>>>>> 90fab5c (2022/7/17/14.12)

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
<<<<<<< HEAD
        // let green =
=======
         let green = ("green","#00FF00");
>>>>>>> 90fab5c (2022/7/17/14.12)

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
<<<<<<< HEAD
        // let unit_struct =
=======
         let unit_struct = UnitStruct;
>>>>>>> 90fab5c (2022/7/17/14.12)
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
