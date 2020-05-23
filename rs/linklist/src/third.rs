use std::rc::Rc;

// 构造一个不可变链表
// 使用不可变链表的原因：实现在多线程之间共享数据

// Arc 和 Rc 的区别：Arc 对引用计数的修改时原子性的，即 Arc 是线程安全的
// Rust models thread-safety in a first-class way with two traits: Send and Sync.
// A type is Send if it's safe to move to another thread. A type is Sync if it's
// safe to share between multiple threads.
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List{ head: Some(Rc::new(Node {
            elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        // List{head: self.head.as_ref().map(|node| node.next.clone())}
        // 用 map 会报错：expected struct `std::rc::Rc`, found enum `std::option::Option`

        // and_then()：Returns [`None`] if the option is [`None`], otherwise calls `f` with the
        // wrapped value and returns the result.
        List{head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        // self.head 的类型是 Rc<Node<T>>，最终我们需要返回一个 Option<&T>
        // 直接调用 self.head.map(|node| &node.elem)，此时 node 的类型是 Rc<Node<T>>，返回 &node.elem
        // 会导致返回一个栈上的值的指针，报错
        self.head.as_ref().map(|node| &node.elem) // 注意返回值的 & 不能漏
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // 根据 Iter 结构体的定义，next Option 的内层值的类型是 &Node<T>
            // node 是 &Rc<Node<T>>，*node 是 Rc<Node<T>>，**node 是 Node<T>，所以 &**node 才是对的
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node)}
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // Returns the inner value, if the `Rc` has exactly one strong reference.
            // Otherwise, an [`Err`][result] is returned with the same `Rc` that was
            // passed in.
            // This will succeed even if there are outstanding weak references.
            // try_unwrap 返回的是 Result
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}