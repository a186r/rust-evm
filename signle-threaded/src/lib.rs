use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool{
    works: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

//trait FnBox{
//    fn call_box(self: Box<Self>);
//}
//
//impl<F: FnOnce()> FnBox for F {
//    fn call_box(self: Box<F>) {
//        (*self)()
//    }
//}

//struct Job;
//将Job改造成类型别名,类型别名能将长的类型变短
type Job = Box<dyn FnOnce() + Send + 'static>;

//Message要么时存放了线程需要运行的Job的NewJob成员，要么是会导致线程退出循环并终止的Terminate成员。
enum Message{
    NewJob(Job),
    Terminate,
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

//实现接口
impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        /// 创建线程池
        ///
        /// 线程池中的线程数量
        ///
        /// #Panics
        ///
        /// 'new'函数在size为0时会panic
        assert!(size > 0);
        /// with_capacty预先分配空间
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut works = Vec::with_capacity(size);
        for id in 0..size{
//            创建线程并将他们存储在vector中
            works.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            works,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
//        在通道中发出任务
        self.sender.send(Message::NewJob(job)).unwrap();
    }

}

//    当线程池被丢弃时，应该join所有线程以确保他们完成其操作。
impl Drop for ThreadPool{
//    self本身是一个可变引用，而且也需要能够修改worker
    fn drop(&mut self) {

        println!("Sending terminate to all workers.");

        for _ in &mut self.works {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.works{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
//            当线程池离开作用域时join每个线程
//            worker.thread.join().unwrap();
        }
    }
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
//        空闭包创建一个thread
//    我们希望闭包一直循环，向通道的接收端请求任务，并在得到任务时执行他们
        let thread = thread::spawn(move || {
            loop {
//                首先调用locker来获取互斥器，如果锁定了互斥器，接着调用recv从通道中接收Job。
//                调用recv会阻塞当前线程，所以如果还没有任务，其会等待知道有可用的任务。
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
//                        job.call_box();
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}