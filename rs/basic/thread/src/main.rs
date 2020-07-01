/**
    标准库中提供的线程模型是 1:1 模型
*/

/**
    并发相关的两个重要 trait：Sync 和 Send
     - Allowing Transference of Ownership Between Threads with Send
        - Send marker trait 表示实现了 Send trait 的类型，其所有权可以在线程之间传递
        - 几乎所有 rust 的类型都是 Send，例外如：Rc<T>
        - 任何完全由 Send 类型组合成的类型会自动被标记为 Send
        - 几乎所有原生类型都是 Send，除了裸指针（raw pointer）
     - Allowing Access from Multiple Threads with Sync
        - Sync marker trait 表示实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用
        - 即：任意类型 T，如果 &T 实现了 Send，则 T 就实现了 Sync
        - 原生类型是 Sync，完全由 Sync 类型组合成的类型也是 Sync
        - Rc<T> 不是 Sync，RefCell<T> 和 Cell<T> 族类型也不是 Sync
        - Mutex<T> 是 Sync
*/

use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程
    // thread::spawn() 的返回值类型是 JoinHandle，该类型是一个有所有权的值，有一个 join 方法
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 阻塞直到线程运行结束
    handle.join().unwrap();

    // move closure
    let v = vec![1, 2, 3];

    // 闭包中使用了 vector，如果不加上 move 关键字获取所有权的话，由于 println! 只需要
    // v 的引用，故程序会尝试获取 v 的引用传入闭包。存在的问题是：程序不知道变量 v 什么时
    // 候会变为非法，因此需要加上 move 关键字转移所有权。
    // 也可以从函数签名上来看
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // {
    //     Builder::new().spawn(f).expect("failed to spawn thread")
    // }
    // 输入和输出参数都加上了 Send 和 'static 的限制
    // Send marker 表示的是可以安全的在线程之间传递值，而不是引用。安全传递引用的 marker 是 Sync
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
