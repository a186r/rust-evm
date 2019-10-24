////////////////长方形面积计算程序
//////////////
//////////////fn main() {
////////////////    println!("Hello, world!");
//////////////    let width1 = 30;
//////////////    let height1 = 50;
//////////////
//////////////    println!(
//////////////        "The area of the rectangle is {} square pixels.",
//////////////        area(width1, height1)
//////////////    );
//////////////
//////////////}
//////////////
//////////////fn area(width: u32, height: u32) -> u32{
//////////////    width * height
//////////////}
////////////
//////////////使用元组的版本
////////////fn main() {
////////////    let rect1 = (30, 50);
////////////
////////////    println!(
////////////        "The area of the rectangle is {} square pixels.",
////////////        area(rect1)
////////////    );
////////////}
////////////
////////////fn area(dimensions: (u32, u32)) -> u32{
////////////    dimensions.0 * dimensions.1
////////////}
//////////
//////////
////////////使用元组的版本会模糊意义，使用结构体，可以更明确地定义我们在做什么
//////////
//////////struct Rectangle{
//////////    width: u32,
//////////    height: u32,
//////////}
//////////
//////////fn main() {
//////////    let rect1 = Rectangle{width: 50, height: 50};
//////////
//////////    println!(
//////////        "The area of the rectangle is {} square pixels.",
//////////        area(&rect1)
//////////    );
//////////}
//////////
////////////我们希望借用结构体而不是获取它的所有权，这样main函数就可以保持rect1的所有权并可以继续使用它
//////////fn area (rectangle: &Rectangle) -> u32 {
//////////    rectangle.width * rectangle.height
//////////}
////////
//////////使用trait来派生
//////////加上注解，以便使用:?
////////#[derive(Debug)]
////////struct Rectangle{
////////    width: u32,
////////    height: u32,
////////}
////////
////////fn main() {
////////
////////    let rect1 = Rectangle{width: 30, height: 50};
////////    println!("rect1 is {:#?}", rect1);
////////}
////////
////////
//////
////////改写成一个定义于Rectangle结构体上的area方法
//////#[derive(Debug)]
//////struct Rectangle{
//////    width: u32,
//////    height: u32,
//////}
//////
//////impl Rectangle{
//////    fn area(&self) -> u32{
//////        self.width * self.height
//////    }
//////
//////    fn add(&self) -> u32 {
//////        self.width + self.height
//////    }
//////}
//////
//////fn main() {
//////    let rect1 = Rectangle{width: 30, height: 50};
//////
//////    println!(
//////        "The area of the rectangle is {} square pixels.",
//////        rect1.area()
//////    );
//////
//////    println!(
//////        "The area of the rectangle is {}",
//////        rect1.add()
//////    );
//////}
//////
//////
//////让一个Rectangle的实例获取另一个Rectangle实例，如果self能完全包含第二个长方形则返回true;否则返回false
////
////#[derive(Debug)]
////struct Rectangle{
////    width: u32,
////    height: u32,
////}
////
////impl Rectangle{
////    fn area(&self) -> u32{
////        self.width * self.height
////    }
////
////    fn can_hold(&self, other: &Rectangle) -> bool{
////        self.width > other.width && self.height > other.height
////    }
////}
////
////fn main() {
////    let rect1 = Rectangle{width: 30, height: 50};
////
////    let rect2 = Rectangle{width: 10, height: 40};
////
////    let rect3 = Rectangle{width: 60, height:45};
////
////    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
////    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
////
////    println!(
////        "The area is {}",
////        rect1.area()
////    );
////
////}
////
////
//#[derive(Debug)]
//struct Rectangle{
//    width: u32,
//    height: u32,
//}
//
//impl Rectangle{
//    fn square(size: u32) -> Rectangle{
//        Rectangle{width: size, height: size}
//    }
//}
//
//fn main() {
////    使用结构体名和::语法来调用这个关联函数
//    let sq = Rectangle::square(3);
//}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}