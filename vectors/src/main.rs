//////////////////fn main() {
////////////////////    创建一个新vector, 可以直接用new
////////////////////    创建一个vec来存储i32类型的值
//////////////////    let v: Vec<i32> = Vec::new();
//////////////////    println!("Hello, world!");
//////////////////
//////////////////    let mut v = Vec::new();
//////////////////    v.push(5);
//////////////////    v.push(6);
//////////////////    v.push(7);
////////////////////    v.push(8);
//////////////////} // 这里v离开作用域被丢弃
//////////////////
////////////////fn main() {
////////////////    let v = vec![1, 2, 3, 4, 5];
////////////////
//////////////////    使用索引语法或者get方法来访问vector中的项
////////////////    let third: &i32 = &v[2];
////////////////    println!("The third element is {}", third);
////////////////
////////////////    match v.get(2) {
////////////////        Some(third) => println!("The third element is {}", third),
////////////////        None => println!("There is no third element."),
////////////////    }
////////////////}
//////////////fn main() {
//////////////    let v = vec![1, 2, 3, 4, 5];
//////////////
////////////////    let does_not_exist = &v[100];
//////////////    let does_not_exist = v.get(100);
//////////////    println!("does not exist is {}", does_not_exist);
//////////////}
////////////fn main() {
////////////    let mut v = vec![1, 2, 3, 4, 5, 6];
////////////
////////////    let first = &v[0];
////////////
////////////    v.push(7);
////////////
////////////    println!("The first element is: {}", first);
////////////}
//////////fn main() {
//////////    let v = vec![1, 2, 3, 4, 5];
//////////
////////////    使用for循环来获取i32值的vector中的每一个元素的不可变引用并将其打印
//////////    for i in &v{
//////////        println!("{}", i);
//////////    }
//////////
//////////    let mut v1 = vec![1, 2, 3, 4, 5];
//////////
////////////    遍历可变vector的每一个元素的可变引用以便能改变他们，给每一个元素+50
//////////    for i in &mut v1{
////////////        为了修改可变引用所指向的值，在使用+=运算符之前解引用运算符*获取i中的值
//////////        *i += 50;
//////////    }
//////////}
////////enum SpreadsheetCell{
////////    Int(i32),
////////    Float(f64),
////////    Text(String),
////////}
////////
////////fn main() {
////////    let row = vec![
////////        SpreadsheetCell::Int(3),
////////        SpreadsheetCell::Text(String::from("bule")),
////////        SpreadsheetCell::Float(10.12),
////////    ];
////////}
//////fn main() {
//////    let mut s = String::new();
//////
//////    let data = "initial contents";
//////
//////    let s = data.to_string();
//////
////////    该方法也可以直接用于字符串字面值
//////    let s1 = "initial contents".to_string();
//////
////////    也可以使用String::from函数来从字符串字面值创建String
//////    let s2 = String::from("initial contents");
//////}
////fn main() {
////    let mut s = String::from("foo");
////
////    s.push_str("bar");
////
////    println!("s is {}", s);
////}
//
//fn main() {
//    let s1 = String::from("s1");
//    let s2 = String::from("s2");
//    let s3 = String::from("s3");
//
////    format宏不会获取任何参数的所有权
//    let s = format!("{}-{}-{}", s1, s2, s3);
//
//    println!("s is {}", s);
//
//    let len = String::from("Hola").len();
//}

fn main() {
//    chars方法会将其分开并返回6个char类型的值
    for c in "नमस्ते".chars(){
        println!("{}", c);
    }

//    bytes方法返回每一个原始字节
    for b in "नमस्ते".bytes(){
        println!("{}", b);
    }
}