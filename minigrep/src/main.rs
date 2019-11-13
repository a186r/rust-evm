use std::env;

//grep 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行
fn main() {
//    读取任何传递给minigrep的命令行参数并将其收集到一个vector中
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}