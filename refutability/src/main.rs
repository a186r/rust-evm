//////////fn main() {
//////////    let some_option_value: Option<i32> = None;
////////////    使用if let "修复"可反驳模式，如果匹配到None，{}内的代码不会被执行
//////////    if let Some(x) = some_option_value {
//////////        println!("{}", x);
//////////    }
//////////}
////////
////////fn main() {
////////    let x = 1;
////////
////////    match x {
////////        1 => println!("one"),
////////        2 => println!("two"),
////////        3 => println!("three"),
////////        _ => println!("anything"),
////////    }
////////}
//////
//////fn main() {
//////    let x = Some(5);
//////    let y = 10;
//////
//////    match x {
//////        Some(50) => println!("Got 50"),
//////        Some(y) => println!("Matched, y = {:?}", y),
//////        _ => println!("Default case, x = {:?}", x),
//////    }
//////
//////    println!("at the end: x = {:?}, y = {:?}", x, y);
//////}
////
////fn main() {
////    let x = 1;
////
////    match x {
////        1 | 2 => println!("one or two"),
////        3 => println!("three"),
////        _ => println!("anything"),
////    }
////}
//fn main() {
////    let x = 5;
////
////    match x {
////        1 ... 5 => println!("one through five"),
////        _ => println!("something else"),
////    }
//    let x = 'c';
//
//    match x {
//        'a' ... 'j' => println!("early ASCII letter"),
//        'k' ... 'z' => println!("late ASCII letter"),
//        _ => println!("something else"),
//    }
//
//}

//可以用模式来结构结构体
struct Point{
    x: i32,
    y: i32,
}

//fn main() {
//    let p = Point{x: 0, y: 7};
////    带有模式的let语句来结构结构体
////    let Point{x: a, y: b} = p;
//    let Point{x, y} = p;
//    assert_eq!(0, x);
//    assert_eq!(7, y);
////    assert_eq!(0, a);
////    assert_eq!(7, b);
//}
fn main() {
    let p = Point{x: 0, y: 7};

    match p {
        Point{x, y: 0} => println!("On the x axis at {}", x),
        Point{x: 0, y} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}