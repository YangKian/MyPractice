use std::mem;

// 额外引入 List 结构体是因为：如果我们声明 pub enum Link，则因为其需要访问 Node，
// 故也需要将 Node 声明为 pub，破坏了封装
pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty, // 优化：没有指定 tag，不需要为 tag 部分分配空间
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // 使用 next: self.head 是错误的，会导致 self.head 的 ownership 被转移
            // std::mem：第一个参数 dst 是可变引用，第二个参数 rsc 是可变值，该函数用第二个参数替换第一个参数，
            // 并返回第一个参数 dst 的旧值。此时第二个参数 rsc 的所有权被转移，不再可用
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 错误的写法
        // let result;
        // // 注意这里要使用引用，因为模式匹配会尝试将要匹配的内容移动到分支中
        // // 所以这里如果使用 self.head 的话，因为 Box<Node> 没有实现 copy trait，所以会出错
        // match &self.head {
        //     Link::Empty => {
        //         result = None
        //     }
        //     Link::More(node) => {
        //         result = Some(node.elem);
        //         // 出错，我们想要 move node 的值（&self.head 的值），但是这个值是个
        //         // 可变引用，不能单向 move，只能 raplace
        //         self.head = node.next;
        //     }
        // };
        // result

        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, // 注意这里是逗号
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

//indicate that the whole test module should only be
// compiled if we're running tests.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}