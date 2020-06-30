use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

mod lib;

// 结合使用 Rc<T> 和 RefCell<T> 实现 Multiple owners of mutable data
// 通过使用 RefCell<T>，我们得到了一个表面上看似不可变的 List 值，通过 RefCell<T>
// 提供的方法来访问它的 interior mutability，因此可以修改我们的数据
// 运行时 borrowing rules 检查防止出现数据竞争
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // rust 的自动解引用机制，value.borrow_mut() 等效于 (*value).borrow_mut()
    // *value.borrow_mut 中的 * 实际上是对 borrow_mut 方法返回的 RefMut 智能指针解引用
    *value.borrow_mut() += 10; // 等效于 *(*value).borrow_mut()

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
