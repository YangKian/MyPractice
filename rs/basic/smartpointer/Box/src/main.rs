/**
    - 智能指针：一种数据结构，除了指针自身的属性外，还有其他额外的元数据和能力
    - references are pointers that only borrow data; in contrast,
      in many cases, smart pointers own the data they point to.
    - 智能指针通常用结构体实现，同时智能指针还实现了 Deref 和 Drop trait
    - 常用智能指针：
        - Box<T> for allocating values on the heap
        - Rc<T>, a reference counting type that enables multiple ownership
        - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces
          the borrowing rules at runtime instead of compile time
*/

/**
    Box<T> 的使用场景：
      - When you have a type whose size can’t be known at compile time and you
        want to use a value of that type in a context that requires an exact size
      - When you have a large amount of data and you want to transfer ownership
        but ensure the data won’t be copied when you do so
      - When you want to own a value and you care only that it’s a type that
        implements a particular trait rather than being of a specific type

    when a box goes out of scope, as b does at the end of main, it will be deallocated.
    The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).
*/

use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 智能指针实现了 Deref trait，可以将指针当成引用来用，还可以自定义解引用的行为
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 实现 MyBox
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(T)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type for the Deref trait to use.

    fn deref(&self) -> &T {
        &self.0
    }
}