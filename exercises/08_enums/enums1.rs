// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit(String),
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Quitting!")));
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
