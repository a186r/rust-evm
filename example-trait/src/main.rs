use core::fmt::Display;

////use example_trait::{Tweet, Summary, NewsArticle};
////use core::fmt::{Display, Debug};
////
////fn main() {
////    let tweet = Tweet{
////        username: String::from("horse_ebooks"),
////        content: String::from("of course, as you probably already know, people"),
////        reply: false,
////        retweet: false,
////    };
////
////    println!("1 new tweet: {}", tweet.summarize());
////
//////    指定一个空的impl块，打印默认实现
////
//////    let article = NewsArticle{
//////        headline: String::from("Penguins win the Stanley CUp Championship!"),
//////        location: String::from("Pittsburgh, PA, USA"),
//////        author: String::from("Iceburgh"),
//////        content: String::from("The Pittsburgh Penguins once again are the best
//////hockey team in the NHL."),
//////    };
//////
//////    println!("New article available! {}", article.summarize());
////    notify(tweet);
////}
////
//////trait作为参数
//////impl trait 适用于短小的句子
//////pub fn notify(item: impl Summary) {
//////    println!("Breaking news! {}", item.summarize());
//////}
////
//////trait bound
//////trait bound与泛型参数声明在一起，位于尖括号中的冒号后面
//////pub fn notify<T: Summary>(item: T) {
//////    println!("Breaking news! {}", item.summarize());
//////}
////
//////需要获取两个都实现了Summary的类型
//////pub fn notify(item1: impl Summary, item2: impl Summary) {}
////
//////item1和item2允许是不同类型的情况
//////pub fn notify<T: Summary>(item1: T, item2: T){}
////
//////通过+指定多个trait
//////pub fn notify(item: impl Summary + Display){}
////
//////上面的语法也适用于泛型的trait bound:
//////pub fn notify<T: Summary + Display>(item: T){}
////
//////使用where语句简化 trait bound语句
//////fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32{
//////
//////}
////// 可以像这样使用where从句,这个函数签名就不会显得那么杂乱，函数名、参数列表、返回值类型都离得很近，
////// 看起来类似没有很多trait bound函数
////fn some_function<T, U>(t: T, u: U) -> i32
////    where T: Display + Clone,
////          U: Clone + Debug
////{}
////
//////也可以在返回值中使用impl trait语法，来返回实现了某个trait的类型
//////这个签名表明，我要返回某个实现了Summary trait 的类型，但是不确定其具体的类型
////fn returns_summarizable() -> impl Summary{
////    Tweet{
////        username: String::from("horse_ebooks"),
////        content: String::from("of course, as you probably already know, people"),
////        reply: false,
////        retweet: false,
////    }
////}
////
//////将largest的签名修改为如下：
////fn largest<T: PartialOrd>(list: &[T]) -> T{
////
////}
//
////在T的trait bound中增加Copy
////一个可以用于任何实现了PartialOrd和Copy trait的泛型的largest函数
//fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
//    let mut largest = list[0];
//
//    for &item in list.iter(){
//        if item > largest{
//            largest = item;
//        }
//    }
//
//    largest
//}
//
//fn main() {
//    let number_list = vec![23, 45, 65, 67, 90];
//
//    let result = largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['b', 'c', 'z', 's'];
//
//    let result = largest(&char_list);
//    println!("The largest char is {}", result);
//}
//有条件的实现方法
struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {

//    我们可以对任何实现了Display trait的类型调用由ToString定义的to_string方法,因为整型实现了Display
    let s = 3.to_string();
    println!("s is {}", s);
}