use rand::Rng;
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

#[derive(Clone, Default)]
pub struct SemaPlusPlus<T> {
    // 通常使用 Arc 和元组来封装条件变量和其对应的锁
    queue_and_cv: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> SemaPlusPlus<T> {
    pub fn new() -> Self {
        SemaPlusPlus {
            queue_and_cv: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn send(&self, message: T) {
        let (queue_lock, cond_var) = self.queue_and_cv.borrow();
        let mut queue = queue_lock.lock().unwrap(); // 先加锁
        queue.push_back(message);
        if !queue.is_empty() {
            // 满足条件后，唤醒所有等待锁的线程
            cond_var.notify_all();
        }
    }

    pub fn recv(&self) -> T {
        let (queue_lock, cond_var) = self.queue_and_cv.borrow();
        // wait_while(Lock, condition)，等待直到被唤醒，同时满足 condition 为 false
        // 注意，wait_while 中实际上会执行先解锁，后加锁的操作，所以需要传入锁对象的所有权，该方法最后会将解锁后的
        // 锁对象重新返回给用户
        let mut queue = cond_var
            .wait_while(
                queue_lock.lock().unwrap(),
                |queue| queue.is_empty()
            )
            .unwrap();
        queue.pop_front().unwrap()
    }
}

fn rand_sleep() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 30)));
}

const NUM_THREADS: usize = 12;
fn main() {
    let sem = SemaPlusPlus::new();
    let mut handles = Vec::new();
    for i in 0..NUM_THREADS {
        let sem_clone = sem.clone();
        let handle = thread::spawn(move || {
            rand_sleep();
            sem_clone.send(format!("Thread {} just finished!", i));
        });
        handles.push(handle);
    }

    for _ in 0..NUM_THREADS {
        println!("{}", sem.recv())
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
