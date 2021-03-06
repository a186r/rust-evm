//fn main() {
//    let favorite_color: Option<&str> = None;
//    let is_tuesday = false;
//    let age: Result<u8, _> = ("34").parse();
//
//    if let Some(color) = favorite_color{
//        println!("Using your favorite color, {}, as the background", color);
//    }else if is_tuesday{
//        println!("Tuesday is green day!");
//    }else if let Ok(age) = age{
//        if age > 30 {
//            println!("Using purple as the background color");
//        }else{
//            println!("Using orange as the background color");
//        }
//    }else{
//        println!("Using blue as the background color");
//    }
//
////    while let循环
//    let mut stack = Vec::new();
//    stack.push(1);
//    stack.push(2);
//    stack.push(3);
//
////    使用while let循环，只要stack.pop返回Some就打印出值
//    while let Some(top) = stack.pop(){
//        println!("{}", top);
//    }
//}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}