use std::thread;
use std::sync::{mpsc, Arc, Mutex};

/**
    线程池的设计思路：
     - 常规线程创建：thread::spawn(| |) -> JoinHandle<T> 接受一个闭包，在线程中
       运行闭包内的代码
     - 线程池需要存储创建好的线程，线程在线程池中等待，只有在调用时才将闭包代码传入执行
    这就需要一个中介来负责沟通线程池和线程，由此引入 Worker 角色
     - 每个 Worker 都存储了一个 JoinHandle<T> 的实例，对应一个线程，由 id 来标记
     - 线程池存储所有 Worker 的集合
    最后，线程池需要维护一个队列，将待执行的任务放入其中，顺次将任务分发给线程执行。这就
    意味着，我们需要一种方式，将待执行的任务发送给 Worker，由此引入 Job 角色
     - 队列的实现可以用 channel 来模拟，线程池持有发送端，每个 Worker 都持有一个接收
       端。由此实现线程池与 Worker 的交互
     - 待执行的任务抽象为 Job，在线程池和 Worker 之间传递
*/

// 引入 Message 类型，实现区分线程池发送给线程的是待执行的任务还是关断指令，实现
// 优雅的关停任务
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    // 对于线程来说，执行闭包中的代码，并将返回值封装在 JoinHandle<T> 中，
    // 由于我们的设计里线程执行完任务后不需要返回值，所以将泛型 T 设置为单元类型 ()
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        // 调用 spawn 方法创建一个新线程，传入通道的接收端
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        // 注意：错误的写法：
        // let thread = thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {} got a job; executing.", id);
        //
        //         job();
        //     }
        // });
        // 存在的问题：Mutex 结构体没有提供 unlock 的 API，unlock 的行为依赖于 lock() 调用
        // 返回的 MutexGuard<T> 的生命周期。在 while let 语句中，MutexGuard<T> 会一直有效
        // 直到 job() 方法执行完毕后发起一轮新的循环，这就导致了解锁操作的延迟
        // 而上面的 let 语句则不同，调用 let 语句后，MutexGuard<T> 在 recv().unwrap()
        // 调用完成后被抛弃，也就意味着 let 语句一结束马上就实现了解锁，锁定的范围仅限于调用 recv()
        // 的期间，避免了在 job() 执行期间还持有锁

        Self {
                id,
                thread: Some(thread),
            }
    }
}

// 使用类型别名
// Job 是待执行任务的抽象，这里实现为一个 trait 对象，持有发送给 execute 方法
// 的闭包的类型
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        // 线程池的线程数应该要大于0，否则 panic
        assert!(size > 0);

        // 创建一个通道来模拟队列
        // 存在的问题：rust 中的 channel 是多生产者单消费者模型，也就是说，没办法通过
        // clone 方法来生成多个消费者端。
        // 解法：在多个线程之间共享同一个接收端通道，使用 Arc<Mutex<T>>
        // Arc 保证多线程之间共享变量，Mutex 保证同一时间只能有一个线程访问变量
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
    // FnOnce represents a closure that takes
    // no parameters and returns the unit type ().
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // 关断的步骤分为两步，首先是通知各个线程结束任务，然后才是等待所有线程结束，销毁资源
        // 如果不实现结束任务通知，线程的执行时在 loop 死循环中进行的，导致就算后续调用了
        // thread.join 也无法停止线程，会一直等待，变成死循环
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 调用 join() 方法结束线程，存在一个问题：join() 方法的参数是 self 不是引用，
            // 也就是说会导致所有权的转移（转移到临时变量 worker）。因此我们需要将线程从其拥
            // 有者 Worker 中提取出来。用 Option 来实现。
            // Option 有一个 take 方法，可以将 Some(T) 中的值移出，并放回一个 None 变量
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}