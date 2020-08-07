use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main() {
    let m = Mutex::new(5);
    {
        // 调用 lock 返回一个 LockResult<MutexGuard<'_, T>>，封装了 MutexGuard
        //  - MutexGuard 是智能指针，实现了 Deref 来指向内部数据，
        //    该智能指针同时还有一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
        //  - LockResult 是一个 Result 对象的类型别名
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m={:?}", m);

    // 多线程共享变量
    // 1.共享变量的所有权问题
    // 2.共享变量要保持原子性
    // 在这里不能使用 Rc<T>，因为 Rc<T> 只解决了所有权的管理问题，并没有使用同步原语来解决原子性问题
    // 应该使用 Arc<T>：Atomic Reference Counting
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 注意：counter 是不可变的，使用 Mutex 之后我们得到了一个 counter 的可变引用，也就是说，Mutex<T>
        // 也有和 RefCell<T> 类似的 interior mutability 的能力
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
