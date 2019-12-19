use std::thread;
use std::sync::mpsc;

pub struct ThreadPool{
    works: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

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
        let mut works = Vec::with_capacity(size);
        for id in 0..size{
//            创建线程并将他们存储在vector中
            works.push(Worker::new(id));
        }

        ThreadPool{
            works
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

impl Worker{
    fn new(id: usize) -> Worker{
//        空闭包创建一个thread
        let thread = thread::spawn(|| {});
        Worker{
            id,
            thread,
        }
    }
}