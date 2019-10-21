////////fn main() {
//////////    println!("Hello, world!");
////////
//////////    let s = "hello";
//////////
//////////    let mut s = String::from("Hello");
//////////
//////////    s.push_str(", World!");
//////////
//////////    println!("{}", s);
////////
//////////    复制（copy）
//////////    let s1 = String::from("hello");
//////////    let s2 = s1;
////////
//////////克隆
//////////    let s1 = String::from("hello");
//////////    let s2 = s1.clone();
//////////
//////////    println!("s1 = {}, s2 = {}", s1, s2);
////////
//////////    所有权与函数
////////
////////}
////////
////////fn gives_ownership() -> String{
////////    let some_string = String::from("hello");
////////
////////    some_string
////////}
////////
////////fn takes_and_gives_back(a_string: String) -> String{
////////    a_string
////////}
//////
//////fn main() {
//////    let s1 = String::from("hello");
//////    let (s2, len) = calculate_length(s1);
//////    println!("The length of '{}' is {}.", s2, len);
//////}
//////
//////fn calculate_length(s: String) -> (String, usize){
//////    let length = s.len();
//////    (s, length)
//////}
////
////fn main() {
////    let s1 = String::from("hello");
////
////    let len = calculate_length(&s1);
////
////    println!("The length of '{}' is {}.", s1, len);
////
////}
////
////fn calculate_length (s :&String) -> usize{
////    s.len()
////}
//
//fn main() {
////    s必须是可变变量
//    let mut s = String::from("hello");
//
////    传入可变引用 &mut s
//    change(&mut s);
//}
//
////函数change接收一个可变引用&mut String
//fn change (some_string: &mut String){
//    some_string.push_str(", world");
//}
//


fn main() {
    let refrence_to_nothing = dangle();
    println!("{}", refrence_to_nothing);
}

fn dangle() -> String{
//    因为s是在这个函数里被创建的，当代码执行完毕后，就会被释放，但是我们尝试返回它的引用，
//    这意味着这个引用会指向一个无效的String，Rust不允许这么做
    let s = String::from("hello");
//返回s的引用
//    &s
//    这里的解决方法是直接返回String，这样所有权就被移动出去了，没有值被释放
    s
} // 这里s离开作用域并被丢弃，其内存被释放



