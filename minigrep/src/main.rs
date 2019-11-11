use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

//    程序的名称占据了第一个值&args[0]
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
