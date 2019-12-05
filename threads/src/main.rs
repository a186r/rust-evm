//use std::thread;
//use std::time::Duration;
//
//fn main() {
////    将子线程存储在变量中
//    let handle =  thread::spawn(|| {
//        for i in 1..10{
//            println!("hi number {} from the spawned thread!", i);
//            thread::sleep(Duration::from_millis(1));
//        }
//    });
//
////    如果将join移到这里，主线程会等待直到新建线程执行完毕之后才开始执行for循环，所以，输出将不会交替出现
//    handle.join().unwrap();
//
//    for i in 1..5{
//        println!("hi number {} from the main thread!", i);
//        thread::sleep(Duration::from_millis(1));
//    }
//
////    对JoinHandle调用join方法时，其会等待线程结束
////    join会阻塞当前线程直到handle所代表的线程结束，阻塞线程意味着组织该线程执行工作或退出
////    handle.join().unwrap();
//}
use std::thread;

fn main() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

//    drop(v);

    handle.join().unwrap();
}