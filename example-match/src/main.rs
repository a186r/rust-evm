use crate::Coin::{Penny, Quarter};
use crate::UsState::Alabama;

fn main() {
    println!("Value in cent {}", value_in_cent(Quarter(Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

//一个枚举
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//枚举成员作为模式的match表达式
fn value_in_cent(coin: Coin) -> u32{
//    match关键字后跟一个表达式，在这里是coin的值
    match coin {
//      match的分支，一个分支有两部分，一个模式和一些代码，，第一个分支的模式是Coin::Penny，
//      而之后=>运算符将模式和将要运行的代码分开，这里的代码仅仅是值1
        Coin::Penny => {
            println!("Locky penny");
            1
        },
//      每一个分支之间,号分开
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }

    let mut count = 0;
}

fn state_quarter(coin: Coin) ->  _ {

    let mut count = 0;

//    match coin {
//        Coin::Quarter(state) => println!("State quarter from {:?}", state),
//        _ => count += 1,
//    }

//    if let Coin::Quarter(state) = coin{
//        println!("State quarter from {:?}", state);
//    } else {
//        count += 1;
//    }

    if let Coin::Quarter(state) = state{
        println!("State quarter from {:?}", state);
    }else{
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => (), // 通配符，_ 模式会匹配所有的值
    }
}

fn some_value() {
//    它匹配一个Option<u8>值并希望当值为3的时候执行代码
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value{
        println!("three");
    }
}
