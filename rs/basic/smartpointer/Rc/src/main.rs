
/**
    Rc<T> 用于 multiple ownership 的场景，Rc<T> 是引用计数的抽象
     - Rc<T> 通过追踪某个值的引用的数量来决定该值是否依旧在被使用
     - We use the Rc<T> type when we want to allocate some data on the heap for multiple
       parts of our program to read and we can’t determine at compile time which part
       will finish using the data last.
    Rc<T> 只能用在单线程场景中
    Rc<T> 只能用于不可变引用的引用计数
*/

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // b -> |3| -> ->
    //              |
    //          a ->|5| -> |10| -> |Nil|
    //              |
    // c -> |4| -> ->
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Rc::cloe() 不会执行深拷贝，只会增加引用计数
    let c = Cons(4, Rc::clone(&a));

    test_count();
}

fn test_count() {
    // 当 Rc<T> 的值离开其作用域时，Drop trait 会自动实现减少引用计数
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
