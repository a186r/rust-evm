use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool{
    works: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

trait FnBox{
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

//struct Job;
//将Job改造成类型别名,类型别名能将长的类型变短
type Job = Box<FnBox + Send + 'static>;

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
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
        self.sender.send(job).unwrap();
    }
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
//        空闭包创建一个thread
//    我们希望闭包一直循环，向通道的接收端请求任务，并在得到任务时执行他们
        let thread = thread::spawn(move || {
            loop {
//                首先调用locker来获取互斥器，如果锁定了互斥器，接着调用recv从通道中接收Job。
//                调用recv会阻塞当前线程，所以如果还没有任务，其会等待知道有可用的任务。
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
        });

        Worker{
            id,
            thread,
        }
    }
}