// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

<<<<<<< HEAD
// I AM NOT DONE
=======
>>>>>>> 90fab5c (2022/7/17/14.12)

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
<<<<<<< HEAD
=======
    Quit,
    Echo(String),
    Move{x:i32,y:i32},
    ChangeColor(i32,i32,i32),

>>>>>>> 90fab5c (2022/7/17/14.12)
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
