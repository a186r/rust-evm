////
////enum IpAddrKind{
////    V4,
////    V6,
////}
////enum IpAddrKind{
////    V4(String),
////    V6(String),
////}
//
//enum IpAddrKind{
//    V4(u8, u8, u8, u8),
//    V6(String),
//}
//
//fn main() {
//    println!("Hello, world!");
//
////    let four = IpAddrKind::V4;
////    let six = IpAddrKind::V6;
////    let home = IpAddrKind::V4(String::from("127.0.0.1"));
////    let loopback = IpAddrKind::V6(String::from("::1"));
//
//    let home = IpAddrKind::V4(127,1,1,1);
//    let loopback = IpAddrKind::V6(String::from("::1"));
//    route(four);
//}
//
//fn route(ip_type: IpAddrKind){}
fn main() {
    let some_number = Some(5);

    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}