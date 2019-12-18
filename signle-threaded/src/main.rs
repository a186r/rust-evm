use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
//    监听传入的流，并在接收到流时打印信息
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
//        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
//    创建一个512字节的缓冲区
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

//    匹配请求并区别处理/请求与其他请求
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{

    }

//    let contents = fs::read_to_string("hello.html").unwrap();
//
////    将一个微型成功HTTP响应写入流
//    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
//
//    stream.write(response.as_bytes()).unwrap();
//    stream.flush().unwrap();
}