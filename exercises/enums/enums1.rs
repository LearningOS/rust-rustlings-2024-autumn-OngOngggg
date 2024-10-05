// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
