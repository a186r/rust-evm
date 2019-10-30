////////////mod front_of_house{
////////////    pub mod hosting{
////////////        pub fn add_to_waitlist() {}
////////////    }
////////////}
////////////
////////////use crate::front_of_house::hosting;
////////////
////////////pub fn eat_at_restaurant() {
////////////    hosting::add_to_waitlist();
////////////    hosting::add_to_waitlist();
////////////    hosting::add_to_waitlist();
////////////}
////////////
////////////fn main(){
////////////
////////////}
////////////
////////////#[cfg(test)]
////////////mod tests {
////////////    #[test]
////////////    fn it_works() {
////////////        assert_eq!(2 + 2, 4);
////////////    }
////////////}
//////////
//////////use std::collections::HashMap;
//////////
//////////fn main() {
//////////    let mut map = HashMap::new();
//////////    map.insert(1, 2);
//////////}
//////////
//////////use std::fmt;
//////////use std::io;
//////////
//////////fn function1() -> fmt::Result{
//////////    Ok(())
//////////}
//////////
//////////fn function2() -> io::Result<()>{
//////////
//////////}
////////use std::fmt::Result;
////////use std::io::Result as IoResult;
////////
////////fn function1() -> Result{
////////    Ok(())
////////}
////////
////////fn function2() -> IoResult<()>{
////////    Ok(())
////////}
//////
//////mod front_of_house{
//////    pub mod hosting{
//////        pub fn add_to_waitlist(){}
//////    }
//////}
//////
//////pub use crate::front_of_house::hosting;
//////
//////pub fn eat_at_restaurant(){
//////    hosting::add_to_waitlist();
//////    hosting::add_to_waitlist();
//////    hosting::add_to_waitlist();
//////}
//////
//////fn main(){
//////
//////}
////
////use rand::Rng;
////
////fn main() {
////    let secret_number = rand::thread_rng().gen_range(1, 101);
////}
//
//use rand::Rng;
//
//fn main() {
//
//}

use rand::Rng;
//使用glob运算符将所有的公有定义引入作用域
use std::collections::*;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1, 101);
}