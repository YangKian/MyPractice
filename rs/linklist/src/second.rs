// 额外引入 List 结构体是因为：如果我们声明 pub enum Link，则因为其需要访问 Node，
// 故也需要将 Node 声明为 pub，破坏了封装
pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// 类型别名
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            // next: mem::replace(&mut self.head, None),
            // .take() 方法：取出 option 中的值，使用 None 来填充原来的位置
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     Link::Empty => None, // 注意这里是逗号
        //     Link::More(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // 错误的用法，map 以值的方式接收 self 作为参数，会导致所有权的转移
        // 之后调用 &node.elem 会报错 returns a reference to data owned by the current function
        // self.head.map(|node| { // node 的类型是 Box<Node<T>>
        // 修正：使用 .as_ref()：Converts from `&Option<T>` to `Option<&T>`.
        self.head.as_ref().map(|node| { // node 的类型是 &Box<Node<T>>
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> { // 实现 Iterator trait
    type Item = T; // 关联类型
    // 需要定义关联类型以及返回 Option<Self::Item> 是因为该接口实际上结合了 has_next 和
    // get_next 的概念。即最终需要一个返回值
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        // Iter 中 next 的类型是 Option<&Node>，而我们通过 map 传入的是 Box<Node>, 最终得到 &Box<Node>，报错
        // Iter {next: self.head.map(|node| &node)}

        // 先对 node 解引用得到 Node 后再重新引用 &*node，还是报错
        // 因为 self.head.map 将 node 的所有权传入了闭包，闭包返回引用导致悬挂指针
        // Iter {next: self.head.map(|node| &*node)}

        // 使用 head.as_ref() 获取引用，再传入 map，此时由于又多了一层引用，因此需要解引用两次
        // Iter {next: self.head.as_ref().map(|node| &**node)}

        // ::<&Node<T>, _> 意味着：需要返回一个 &Node<T>，需要显示指明的原因是我们在闭包中传入了一个 Option<&T>
        // 类型，编译器不能很好的为我们执行自动解引用
        Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node)}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &*node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // 错误的用法，报错：cannot assign twice to immutable variable `value`
        // 在闭包中是用 |&mut value| 的方式无法指定 value 是一个可变引用。
        // 这里实际上创建了一个模式匹配，用来匹配填入到闭包中的参数
        // |&mut value| 的意思是：参数是一个可变引用，但是只需要把它指向的值拷贝到闭包里就可以
        // list.peek_mut().map(|&mut value| {
        //     value = 42 // value 的类型是 i32
        // });

        list.peek_mut().map(|value| { // value的类型是 &mut i32
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}