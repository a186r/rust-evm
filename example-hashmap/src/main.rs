////////////use std::collections::HashMap;
////////////
////////////fn main() {
////////////    println!("Hello, world!");
////////////
////////////    let mut scores = HashMap::new();
////////////
////////////    scores.insert(String::from("Blue"), 10);
////////////    scores.insert(String::from("Yellow"), 50);
////////////
////////////}
//////////
//////////use std::collections::HashMap;
//////////
////////////另一个创建HashMap的方法
//////////fn main() {
////////////    创建一个vector
//////////    let teams = vec![String::from("Blue"), String::from("Yellow")];
////////////    创建另一个vector
//////////    let initial_score = vec![10, 10];
////////////    使用collect方法将这个元组vector转换成一个HashMap
////////////    这里的HashMap<_, _>类型注解是必要的，因为可能collect很多不同的数据结构，
////////////    而除非显示指定，否则Rust无从得知你需要的类型
//////////    let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
//////////}
//////////访问HashMap中的值
////////
////////use std::collections::HashMap;
////////
////////fn main() {
////////    let mut scores = HashMap::new();
////////
////////    scores.insert(String::from("Blue"), 10);
////////    scores.insert(String::from("Yellow"), 50);
////////
////////    let team_name = String::from("Blue");
////////
//////////    因为get返回的Option<V> 所以结果被装进Some 如果某个键在HashMap中没有对应的值，get会返回None
////////    let score = scores.get(&team_name);
////////
////////}
//////
//////use std::collections::HashMap;
//////
//////fn main() {
//////    let mut scores = HashMap::new();
//////
//////    scores.insert(String::from("Blue"), 10);
//////    scores.insert(String::from("Yellow"), 50);
//////
//////    for (key, value) in &scores {
//////        println!("{}:{}", key, value);
//////    }
//////}
////
////use std::collections::HashMap;
////
////fn main() {
////    let mut scores = HashMap::new();
////
////    scores.insert(String::from("Blue"), 10);
////    scores.insert(String::from("Blue"), 50);
////
////    println!("{:?}", scores);
////}
//
//use std::collections::HashMap;
//
////只修改不存在的值
//fn main() {
//    let mut scores = HashMap::new();
//
//    scores.insert(String::from("Blue"), 10);
//
//    scores.entry(String::from("Yellow")).or_insert(50);
//    scores.entry(String::from("Blue")).or_insert(50);
//
//    println!("{:?}", scores);
//}

use std::collections::HashMap;

//根据旧值更新一个值
fn main() {
    let text = "hello world wonderfule world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
//        如果是第一次出现就插入0
        let count = map.entry(word).or_insert(0);
//        为了赋值，必须首先使用*号解引用count，这个可变一你用在for循环的结尾离开作用域
        *count += 1;
    }

    println!("{:?}", map);
}