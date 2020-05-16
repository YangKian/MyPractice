use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程
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

    let v = vec![1, 2, 3];

    // 闭包中使用了 vector，如果不加上 move 关键字获取所有权的话，由于 println! 只需要
    // v 的引用，故程序不知道变量 v 什么时候会变为非法，因此需要加上 move 关键字转移所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
