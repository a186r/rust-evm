//fn main() {
////    println!("Hello, world!");
//
////    let s = "hello";
////
////    let mut s = String::from("Hello");
////
////    s.push_str(", World!");
////
////    println!("{}", s);
//
////    复制（copy）
////    let s1 = String::from("hello");
////    let s2 = s1;
//
////克隆
////    let s1 = String::from("hello");
////    let s2 = s1.clone();
////
////    println!("s1 = {}, s2 = {}", s1, s2);
//
////    所有权与函数
//
//}
//
//fn gives_ownership() -> String{
//    let some_string = String::from("hello");
//
//    some_string
//}
//
//fn takes_and_gives_back(a_string: String) -> String{
//    a_string
//}

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}