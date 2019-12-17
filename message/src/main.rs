enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit....");
        },

        Message::Move {x, y} => {
            println!("Move in x {} and y {}", x, y);
        },

        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change color and r, g, b is {}, {} and {}", r, g, b);
        }
    }
}