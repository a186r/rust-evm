//////////////fn largest_i32(list: &[i32]) -> i32{
//////////////    let mut largest = list[0];
//////////////
//////////////    for &item in list.iter(){
//////////////        if item > largest{
//////////////            largest = item;
//////////////        }
//////////////    }
//////////////
//////////////    largest
//////////////}
//////////////
//////////////fn largest_char(list: &[char]) -> char{
//////////////    let mut largest = list[0];
//////////////
//////////////    for &item in list.iter(){
//////////////        if item > largest {
//////////////            largest = item;
//////////////        }
//////////////    }
//////////////
//////////////    largest
//////////////}
//////////////
//////////////fn main() {
//////////////    let number_list = vec![12,34,45,6324,67,23];
//////////////
//////////////    let result = largest_i32(&number_list);
//////////////
//////////////    println!("The largest number is {}", result);
//////////////
//////////////    let char_list = vec!['z', 's', 'c', 't', 'i'];
//////////////
//////////////    let result = largest_char(&char_list);
//////////////    println!("The largest number is {}", result);
//////////////}
//////////////一个暂时不能编译的泛型参数的largest函数定义
////////////
////////////fn largest<T>(list: &[T]) -> T{
////////////    let mut largest = list[0];
////////////
////////////    for &item in list.iter(){
////////////        if item > largest{
////////////            largest = item;
////////////        }
////////////    }
////////////
////////////    largest
////////////}
////////////
////////////fn main() {
////////////    let number_list = vec![30, 50, 70, 90, 101];
////////////
////////////    let result = largest(&number_list);
////////////    println!("The largest number is {}", result);
////////////
////////////    let char_list = vec!['a', 'c', 'd', 's', 'y'];
////////////    let result = largest(&char_list);
////////////
////////////    println!("The largest number is {}", result);
////////////}
//////////
//////////struct Point<T>{
//////////    x: T,
//////////    y: T,
//////////}
//////////
//////////fn main() {
//////////    let integer = Point{x: 5, y: 10};
//////////    let float = Point{x: 1.0, y: 4.0};
//////////}
////////
////////struct Point<T, U> {
////////    x: T,
////////    y: U,
////////}
////////
////////fn main() {
////////    let both_integer = Point{x: 5, y: 10};
////////    let both_float = Point{x: 1.0, y: 4.0};
////////    let integer_and_float = Point{x: 1.0, y: 4};
////////}
//////
//////enum Option<T>{
//////    Some(T),
//////    None,
//////}
//////
////////枚举也可以拥有多个泛型类型
//////enum Result<T, E>{
//////    Ok(T),
//////    Err(E),
//////}
////
//////也可以在定义中使用泛型在结构体和枚举上实现方法
////struct Point<T>{
////    x: T,
////    y: T,
////}
////
////impl<T> Point<T>{
//////    在Point<T>结构体上实现方法x，它返回T类型的字段x的引用
////    fn x(&self) -> &T{
////        &self.x
////    }
////}
////
////fn main() {
////    let p = Point{x: 5, y: 10};
////    let p = Point{x: 1.1, y: 1.3};
////
////    println!("p.x = {}", p.x());
////}
//
////方法使用了与结构体定义中不同类型的泛型
//struct Point<T, U>{
//    x: T,
//    y: U,
//}
//
//impl<T, U> Point<T, U> {
//    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
//        Point{
//            x: self.x,
//            y: other.y,
//        }
//    }
//}
//
//fn main() {
//    let p1 = Point{x: 5, y: 4.4};
//    let p2 = Point{x: "Hello", y: 'c'};
//
//    let p3 = p1.mixup(p2);
//
//    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
//}

//一个类型的行为由其可供调用的方法构成，如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
//trait定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必须的行为的集合。