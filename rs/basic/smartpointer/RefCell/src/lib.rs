/**
    Interior mutability：rust 中的一种设计模式，允许在数据已经有不可变引用的情况下修改数据。通常情况下，
    这种行为是被 borrowing rules 所禁止的。
     - 存在这样一种场景：对于自身的方法来说，某个变量是可变的，但是对于外部的其他代码，该变量是不可变的
    RefCell<T> type represents single ownership over the data it holds
    ReCell<T> V.S. Box<T>
     -  borrowing rules:
        - 任意时刻，只能拥有一个可变引用或任意数量的不可变引用
        - 引用必须总是有效的
     -  对于引用和 Box<T>，borrowing rules‘s invariants 在编译时执行，如果违反了规则，则会得到一个
        编译错误；
     -  对于 RefCell<T>，borrowing rules's invariants 在运行时执行，如果违反了规则，则会 panic
        并退出。
    在运行时检查 borrowing rules 的好处在于：存在某些运行时内存安全的场景，在编译时检查则无法满足要求。
    当编译器无法理解并确认代码的安全性时，如果用户可以保证代码满足 borrowing rules，则可以使用 RefCell<T>
    RefCell<T> 只能用在单线程场景
*/

/**
    Box<T>、Rc<T> 和 RefCell<T> 的选择依据：
     - Rc<T> 允许相同数据有多个 owners，Box<T> 和 RefCell<T> 只能有唯一的 owner
     - Box<T> 允许在编译时检查可变或者不可变引用；
       Rc<T> 只允许在编译时检查不可变引用；
       RefCell<T> 允许在运行时检查可变或者不可变引用；
     - 即使 RefCell<T> 是不可变的，也可以修改 RefCell<T> 中的值；
*/

/**
    RefCell<T> 提供了 borrow 和 borrow_mut 两个安全的API来创建不可变和可变引用
    borrow 方法返回一个 Ref<T> 类型的智能指针，borrow_mut 返回一个 RefMut<T> 类型的智能指针
    RefCell<T> 追踪 Ref<T> 和 RefMut<T> 智能指针的活跃个数，任意时刻 RefCell<T> 允许我们持
    有多个 immutable borrows 或者一个 mutable borrow。如果违反了这些规则，RefCell<T> 会在
    运行时返回一个 panic
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // 在我们的实现中，send 方法会将 message 添加到 sent_messages vector 中，这就要求
        // self 是可变的，但是如果 self 是可变引用就违反了 trait 的函数签名，因此这时选择使用 RefCell
        fn send(&self, message: &str) {
            // 通过调用 .borrow_mut() 方法获取一个 sent_messages 成员的可变引用，也就是一个 vector
            // 之后就可以通过 vector 的方法去修改该成员变量
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // 调用 .borrow() 返回一个成员的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}