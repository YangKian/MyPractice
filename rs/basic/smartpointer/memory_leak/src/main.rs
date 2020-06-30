use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // 树上的每一个结点可能有多个父节点，所以定义 Vec<Rc<Node>>
    // 可能需要修改结点，所以用 RefCell<T> 来封装 Vec<Rc<Node>>
    children: RefCell<Vec<Rc<Node>>>,
    // 不能使用强引用，因为子节点已经是强引用了，再用强引用会引用循环
    // 父节点应当拥有子节点，父节点如果被删除，则子节点也应该被删除
    // 而子节点不应当拥有父节点，子节点如果被删除，不应该导致父节点也被删除
    // 使用弱引用
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // 循环引用导致内存泄漏
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b init rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 修改 a，使其指向 b 而不是 Nil，导致循环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 下面一条语句会导致栈溢出
    // println!("a next item = {:?}", a.tail());

    // 使用 Weak<T> 来防止循环引用
    // Rc::clone 增加一个 Rc<T> 的强引用计数，Rc<T> 的实例在强引用计数归 0 后才会被清除
    // Rc::downgrade 创建一个 Weak<T> 类型的智能指针，增加弱引用计数。
    // 弱引用并不表达所有权关系，不会造成引用循环，因为任何弱引用的循环会在其关联值的强引用计数归
    // 0 后被打破。
    // 由于弱引用可能会被 dropped，所以在操作任何弱引用指向的对象前，都要确认该对象还存在。
    // 对 Weak<T> 的实例调用 upgrade 方法可以返回一个 Option<Rc<T>>，由此可以检查该对象是
    // 否还存在

    // 创建一个叶子结点，值为 3，没有 children
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 创建一个分支结点，值为 5，有一个叶子节点作为 children
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });
    // 为叶子节点设置父节点
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow_mut().upgrade());
}
