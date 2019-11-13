use std::env;

//grep 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行

fn main() {
    let args: Vec<String> = env::args().collect();

//    程序的名称占据了第一个值&args[0]
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
