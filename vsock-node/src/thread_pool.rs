use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建一个线程池
    /// size是池子中线程的数量
    /// panic：size为0则panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers: workers,
            sender: Some(sender),
        }
    }

    // F 类型参数还具有特征边界 Send 和生命周期边界 'static ，
    // 这在我们的情况下很有用：我们需要 Send 将闭包从一个线程转移到另一个线程
    // 'static 因为我们不知道线程需要多长时间才能执行。让我们在 ThreadPool 上创建一个 execute 方法
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    // Mutex 结构没有公共 unlock 方法，因为锁的所有权基于 LockResult<MutexGuard<T>> 的生命周期> lock 方法返回。
    // 在编译时，借用检查器可以强制执行以下规则：除非我们持有锁，否则无法访问由 Mutex 保护的资源。
    // 然而，如果我们不注意 MutexGuard<T> 的生命周期，这种实现也可能导致锁的持有时间比预期的要长
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 需要闭包中永远循环，进行向通道的接收端请求一项作业，并在收到作业时候运行该作业
        let thread = thread::spawn(move || loop {
            // 先通过lock确保互斥状态，获得了锁之后recv从通道接收job
            // Mutex<T> 确保一次只有一个 Worker 线程尝试请求作业。
            let msg = receiver.lock().unwrap().recv();
            // while let不回删除临时值，导致锁在调用job的时候保持lock的状态，造成其他工作线程无法工作
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            match msg {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job()
                }
                Err(err) => {
                    println!("Worker {id} disconnected; shutting down.\terr: {}", err);
                    break;
                }
            }
            // }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
