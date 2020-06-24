use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc stands for multiple producer, single consumer.
    // channel() 方法返回一个元组，第一个值是发送端，第二个值是接收端
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // send 返回一个 Result，如果接收端已经关闭，则返回 error
        tx.send(val).unwrap();
    });

    // 接收端有两个常用方法：recv 和 try_recv
    // recv：阻塞方法，返回值是 Result
    // try_recv：非阻塞方法，立刻返回 Result，err 表示当前没有任何消息
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    multiple_producer();
}

fn multiple_producer() {
    let (tx, rx) = mpsc::channel();

    // 调用 clone 创建一个新的发送端
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
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

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}