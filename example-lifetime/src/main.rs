//////////fn main() {
//////////    println!("Hello, world!");
//////////
////////////    声明了没有初始值的变量，所以这些变量存在与外部作用域
////////////    Rust确实不允许空值
////////////    let r;
////////////
////////////    {
////////////        let x = 5;
////////////        r = &x;
////////////    }
//////////
//////////    let x = 5;
//////////
//////////    let r = &x;
//////////
//////////    println!("r: {}", r);
//////////}
////////
//////////生命周期引用
//////////fn lifet(){
//////////    &i32 // 引用
//////////    &'a i32; // 带有显式生命周期的引用
//////////    &'a mut i32 // 带有显式生命周期的可变引用
//////////}
////////
//////////fn main() {
//////////    let string1 = String::from("abcd");
//////////    let string2 = String::from("ab");
//////////
//////////    let result = longest(string1.as_str(), string2.as_str());
//////////    println!("The longest string is {}", result);
//////////}
////////
////////fn main() {
////////    let string1 = String::from("long string is long");
////////
////////    {
////////        let string2 = String::from("xy");
////////        let result = longest(string1.as_str(), string2.as_str());
////////        println!("The longest string is {}", result);
////////    }
////////}
////////
//////////在离开作用域之后使用result,无法编译通过
//////////fn main() {
//////////    let string1 = String::from("long string is long");
//////////    let result;
//////////    {
//////////        let string2 = String::from("xyz");
//////////        result = longest(string1.as_str(), string2.as_str());
//////////    } //string2已经无效了
//////////
//////////    println!("The longest string is {}", result);
//////////}
////////
//////////函数定义指定了签名中所有的引用必须有相同的生命周期'a
//////////fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
//////////    if str1.len() > str2.len(){
//////////        str1
//////////    }else{
//////////        str2
//////////    }
//////////}
////////
//////////修改为总是返回第一个参数
//////////fn longest<'a>(x: &'a str, y: &str) -> &'a str{
//////////    x
//////////}
////////
//////////悬垂引用，不能编译
//////////即便我们为返回值指定了生命周期参数‘a，编译也会失败，因为返回值的生命周期与参数完全没有关联
////////fn longest<'a>(x: &str, y: &str) -> &'a str{
////////    let result = String::from("really long string");
////////    result.as_str()
////////}// result在这里将离开作用域并被清理，而我们尝试从函数返回一个result的引用
//////////这种情况下最好的解决方案就是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。
//////
////////一个存放引用的结构体，所以其定义需要生命周期注解
//////struct ImportantExcerpt<'a> {
//////    part: &'a str,
//////}
//////
//////fn main() {
//////    let novel = String::from("Call me Ishmael. Some years ago...");
//////    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//////    let i = ImportantExcerpt{part: first_sentence};
//////}
////fn main() {
////
////}
////
//////这个函数没有生命周期注解却能编译
////fn first_word(s: &str) -> &str{
////    let bytes = s.as_bytes();
////
////    for (i, &item) in bytes.iter().enumerate() {
////        if item == b' ' {
////            return &s[0..i];
////        }
////    }
////
////    &s[..]
////}
//
//////方法定义中的生命周期注解
////struct ImportantExcerpt<'a> {
////    part: &'a str,
////}
////
////impl<'a> ImportantExcerpt<'a> {
////    fn level(&self) -> i32 {
////        3
////    }
////}
//
////适用于第3生命周期省略规则的例子
//struct ImportantExcerpt<'a> {
//    part: &'a str,
//}
//
//impl<'a> ImportantExcerpt<'a> {
//    fn announce_and_return_part(&self, announcement: &str) -> &str{
//        println!("Attention please: {}", announcement);
//        self.part
//    }
//}
//
//fn main() {
////    所有的字符串字面值都拥有'static生命周期
////    这个字符串的文本被直接存储在程序的二进制文件中而这个文件总是可用的
////    因此所有的字符串字面值都是static的
//    let s: &'static str = "I have a static lifetime";
//}
//

use std::fmt::Display;

//结合泛型类型参数trait bound和生命周期，在同一个函数中指定泛型类型参数、trait bounds和生命周期的语法
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}