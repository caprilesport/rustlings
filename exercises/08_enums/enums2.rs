// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move((i32, i32)),
    Echo(String),
    ChangeColor((i32, i32, i32)),
    Quit,
}

impl Message {
    fn call(&self) -> String {
        println!("{:?}", self);
        match self {
            Message::Echo(string) => string.to_string(),
            _ => String::from("aha!"),
        }
    }
}

fn main() {
    let messages = [
        Message::Move((10, 2)),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor((200, 255, 255)),
        Message::Quit,
    ];

    for message in &messages {
        println!("testing, value: {}", message.call())
    }
}
