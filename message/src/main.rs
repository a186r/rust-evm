//////enum Color{
//////    Rgb(i32, i32, i32),
//////    Hsv(i32, i32, i32),
//////}
//////
//////enum Message{
//////    Quit,
//////    Move {x: i32, y: i32},
//////    Write(String),
//////    ChangeColor(Color),
//////}
//////
//////fn main() {
//////    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
//////
//////    match msg {
////////        Message::Quit => {
////////            println!("The Quit....");
////////        },
////////
////////        Message::Move {x, y} => {
////////            println!("Move in x {} and y {}", x, y);
////////        },
//////
////////        Message::Write(text) => println!("Text message: {}", text),
//////        Message::ChangeColor(Color::Hsv(h, s, v)) => {
//////            println!("Change color and h, s, v is {}, {} and {}", h, s, v);
//////        },
//////        Message::ChangeColor(Color::Rgb(r, g, b)) => {
//////            println!("r, g, b is {}, {} and {}", r, g, b);
//////        }
//////
//////        _ => {}
//////    }
//////}
////
////fn foo(_: i32, y: i32){
////    println!("This code only print the y parameter: {}", y);
////}
////
////fn main() {
////    foo(10, 12);
////}
//
//fn main() {
//    let mut setting_value = Some(5);
//    let new_setting_value = Some(10);
//
//    match (setting_value, new_setting_value) {
////        当不需要Some中的值时，在模式那使用下划线来匹配Some成员
//        (Some(_), Some(_)) => {
//            println!("Can't overwrite an existing customized value");
//        }
//
//        _ => {
//            setting_value = new_setting_value;
//        }
//    }
//    println!("setting is {:?}", setting_value);
//}

fn main() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}