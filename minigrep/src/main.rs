use std::{env, process};

use minigrep;
use minigrep::Config;

//新的main函数
fn main() {
    let config  = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        process::exit(1);
    }
}