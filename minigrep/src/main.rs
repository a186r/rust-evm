//use std::{env, fs};
//
////grep 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行
//
//fn main() {
//    let args: Vec<String> = env::args().collect();
//
////    程序的名称占据了第一个值&args[0]
//    let query = &args[1];
//    let filename = &args[2];
//
//    println!("Searching for {}", query);
//    println!("In file {}", filename);
//
//    // 打开文件并返回其内容
//    let contents = fs::read_to_string(filename)
//        .expect("Something went wrong reading the file.");
//
//    println!("With text:\n{}", contents);
//}

use std::{env, fs};

//新的main函数
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}",  config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
}

struct Config{
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{
        query,
        filename,
    }
}