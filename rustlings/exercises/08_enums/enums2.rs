#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Size {
    width: u64,
    height: u64,
}

#[derive(Debug)]
enum Message {
    Resize(Size),
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
    // TODO: Define the different variants used below.
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize(Size { width: 10, height: 30 }),
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
