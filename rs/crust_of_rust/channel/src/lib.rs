use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//      - 实现1：Mutex + Condvar + VecDeque
//      - 实现2：Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//      - 实现1：Mutex + Condvar + VecDeque
//      - 实现2：Mutex + Condvar + LinkedList
//      - 实现3：Atomic Linked list
//      - 实现4：Atomic Block Linked list
//  - Rendezvous channels: Synchronous with capacity = 0, Used for thread synchronization.
//  - Oneshot channels: Any capacity. In practice, only one call to send().

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

// 为 Sender<T> 实现 Clone trait，此时不能使用 #[derive(Clone)]
// 因为 #[derive(Clone)] 实际上是：
// impl<T: Clone> for Sender<T>，会将 T 设置为绑定了 Clone trait
// 但是在我们的场景中，Arc 已经实现了 Clone，对 Arc 内部封装的元素 T 不
// 要求也实现 Clone，因此不能直接用导出的方式
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // 每次调用 clone，增加 sender 的计数
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner); // drop the lock

        Sender {
            // 错误的写法：
            // inner: self.inner.clone(),
            // 如果 inner 也实现了 Clone trait, 则 rust 无法知晓此处调用的
            // inner.clone() 是指内部 inner 的 clone 还是 Arc 的 clone
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        // 如果所有的 senders 计数为 0，说明所有的发送端都关闭了，则唤醒接收端退出
        if inner.senders == 0 {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        // 从队尾插入
        inner.queue.push_back(t);
        drop(inner); // drop the lock
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

// 不加 buffer 时存在的问题：每次 recv 都需要加锁
// 优化的原理：只有一个 receiver !!!
// 每次我们调用 recv，首先检查 buffer 中是否有值，如果有则直接从其中弹出值
// 如果 buffer 为空，则加锁，直接将整个发送队列交换到 buffer 中缓存起来，则
// 之后的调用就不再需要加锁执行
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    std::mem::swap(&mut self.buffer, &mut inner.queue);
                    return Some(t)
                },
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

// 未添加 buffer 的实现
// impl<T> Receiver<T> {
//     pub fn recv(&mut self) -> Option<T> {
//         let mut inner = self.shared.inner.lock().unwrap();
//         // 从队头弹出
//         loop {
//             match inner.queue.pop_front() {
//                 Some(t) => return Some(t),
//
//                 // 这里非常容易出错，导致 recv 挂起，可以通过 dbg!() 来输出 inner.senders 的值
//                 // 执行 cargo t -- --test-threads=1 --nocapture
//                 // None if dbg!(inner.senders) == 0 => return None,
//
//                 // 如果此时 senders 计数为 0，则直接退出
//                 None if inner.senders == 0 => return None,
//                 // 如果不使用 senders 字段来追踪引用次数，可以通过 Arc::strong_count() 方法来
//                 // 获取引用此时，此时等于 1，因为 receiver 还持有一个引用
//                 // 如果有弱引用的场景该方法可能会有问题，这里都是强引用，所以可以使用该方法
//                 // 另一个问题是，这种方法要实现最后一个 sender 关闭后唤醒 receiver 会很困难
//                 // 因为仅通过 Arc::strong_count() == 2 来判断的话，有可能关掉的是 receiver 端
//                 // None if Arc::strong_count(&self.shared) == 1 => return None,
//
//                 None => {
//                     inner = self.shared.available.wait(inner).unwrap();
//                 }
//             }
//         }
//     }
// }

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    // VecDeque: 双端队列
    inner: Mutex<Inner<T>>,
    // 信号量
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        // let _ = tx; 错误的写法，会导致 tx 无法调用 drop
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
