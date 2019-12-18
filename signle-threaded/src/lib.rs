pub struct ThreadPool;

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
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}