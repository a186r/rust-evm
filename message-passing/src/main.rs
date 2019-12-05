//////use std::sync::mpsc;
//////use std::thread;
//////
//////fn main() {
////////    创建一个通道，并将其两端赋值给tx和rx
//////    let (tx, rx) = mpsc::channel();
////////    tx.send(()).unwrap();
//////    thread::spawn(move || {
//////        let val = String::from("hi");
////////        在这个栗子里，出错的时候调用unwrap产生panic，但是对于一个真实的程序，需要合理的处理它
//////        tx.send(val).unwrap();
//////    });
//////
////////    在主线程中接收并打印hi
//////    let received = rx.recv().unwrap();
//////    println!("Got: {}", received);
//////}
////
////use std::sync::mpsc;
////use std::thread;
////
////fn main() {
////    let (tx, rx) = mpsc::channel();
////
////    thread::spawn(move || {
////        let val = String::from("hi");
////        tx.send(val).unwrap();
////        println!("val is {}", val);
////    });
////
////    let received = rx.recv().unwrap();
////    println!("Got: {}", received);
////}
//
//use std::sync::mpsc;
//use std::thread;
//use std::time::Duration;
//
//fn main() {
//    let (tx, rx) = mpsc::channel();
//    thread::spawn(move || {
//        let vals = vec![
//            String::from("hi"),
//            String::from("from"),
//            String::from("the"),
//            String::from("thread"),
//        ];
//
//        for val in vals{
//            tx.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
//    for received in rx {
//        println!("Got: {}", received);
//    }
//}

//从多个生产者发送多个消息
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

//    clone生产者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}