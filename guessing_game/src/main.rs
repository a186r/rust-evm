//io库来自于标准库，也被称为std
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

//    生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

//    rust中，变量默认是不可变的，加mut，使得变量成为一个可变变量
    let mut guess = String::new();

//    read_line的工作是，无论用户在标准输入中输入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

//    检测guess类型并转换为u32
//    let guess: u32 = guess.trim().parse().expect("Please type a number");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

//    使用println!占位符打印值
    println!("You guessed:{}", guess);

        println!("Please input your guess.");
//        匹配结果
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
