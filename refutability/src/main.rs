//////fn main() {
//////    let some_option_value: Option<i32> = None;
////////    使用if let "修复"可反驳模式，如果匹配到None，{}内的代码不会被执行
//////    if let Some(x) = some_option_value {
//////        println!("{}", x);
//////    }
//////}
////
////fn main() {
////    let x = 1;
////
////    match x {
////        1 => println!("one"),
////        2 => println!("two"),
////        3 => println!("three"),
////        _ => println!("anything"),
////    }
////}
//
//fn main() {
//    let x = Some(5);
//    let y = 10;
//
//    match x {
//        Some(50) => println!("Got 50"),
//        Some(y) => println!("Matched, y = {:?}", y),
//        _ => println!("Default case, x = {:?}", x),
//    }
//
//    println!("at the end: x = {:?}, y = {:?}", x, y);
//}

fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}