enum Color{
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
//        Message::Quit => {
//            println!("The Quit....");
//        },
//
//        Message::Move {x, y} => {
//            println!("Move in x {} and y {}", x, y);
//        },

//        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color and h, s, v is {}, {} and {}", h, s, v);
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("r, g, b is {}, {} and {}", r, g, b);
        }

        _ => {}
    }
}