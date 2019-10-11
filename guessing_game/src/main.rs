//io库来自于标准库，也被称为std
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

//    rust中，变量默认是不可变的，加mut，使得变量成为一个可变变量
    let mut guess = String::new();

//    read_line的工作是，无论用户在标准输入中输入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed:{}", guess)
}
