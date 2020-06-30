/**
    - 智能指针：一种数据结构，除了指针自身的属性外，还有其他额外的元数据和能力
    - references are pointers that only borrow data; in contrast,
      in many cases, smart pointers own the data they point to.
    - 智能指针通常用结构体实现，同时智能指针还实现了 Deref 和 Drop trait
    - Deref trait：实现 Deref trait 允许我们自定义解引用运算符 * 的行为。通过实现 Deref trait，
      智能指针可以被看作是普通引用来使用
    - Drop trait：实现 Drop trait 允许我们自定义变量离开作用域时的行为。
    - 标准库提供的智能指针：
        - Box<T> for allocating values on the heap
        - Rc<T>, a reference counting type that enables multiple ownership
        - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces
          the borrowing rules at runtime instead of compile time
*/

/**
    Box<T> 在堆上分配空间，栈上存的是指向堆空间的指针，除了堆分配之外，没有其他额外开销
    Box<T> 的使用场景：
      - 当有一个在编译时大小未知的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
      - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
      - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
*/

/**
    函数和方法的隐式强制解引用（Implicit Deref Coercion with Functions and Methods）
     - Deref Coercion 只作用在实现了 Deref trait 的类型上。
     - Deref Coercion 将一种类型强制转换为另一种类型的引用，如：&String 转为 &str
     - Deref coercion happens automatically when we pass a reference to a particular
       type’s value as an argument to a function or method that doesn’t match the
       parameter type in the function or method definition. A sequence of calls to
       the deref method converts the type we provided into the type the parameter needs.
     - Deref coercion 的时间开销用在编译时，不会对运行时增加额外开销

     Deref trait 用于重载不可变引用的解引用运算符，DerefMut trait 用于重载可变引用的解引用运算符
     Rust does deref coercion when it finds types and trait implementations in three cases:
      - From &T to &U when T: Deref<Target=U>
      - From &mut T to &mut U when T: DerefMut<Target=U>
      - From &mut T to &U when T: Deref<Target=U>
*/

use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 自定义 MyBox，实现 Deref trait
struct MyBox<T>(T); // MyBox 是一个元组结构体，只有一个T类型的元素

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The Deref trait, provided by the standard library, requires us to implement
// one method named deref that borrows self and returns a reference to the inner data.
// 没有实现 Deref trait，编译器只能解引用 & 标注的引用类型，Deref trait 提供的 deref 方法可以返回一个
// & 标注的引用类型，让编译器可以正确完成解引用操作。deref 方法选择返回引用类型而不是直接返回值的原因是：
// 如果选择直接返回值，会导致值的所有权被移出 self，而这不是我们所期望的行为
// *y => *(y.deref())
impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type for the Deref trait to use.

    fn deref(&self) -> &T {
        &self.0
    }
}

// 自定义 CustomSmartPointer，实现 Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    // 使用 Box<T> 在堆上存储数据
    // when a box goes out of scope, as b does at the end of main, it will be deallocated.
    // The deallocation happens for the box (stored on the stack) and the data it points
    // to (stored on the heap).
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 智能指针实现了 Deref trait，可以将指针当成引用来用，还可以自定义解引用的行为
    let x = 5;
    // y1 和 y2 的区别：y2 是一个指向 x 的 Box 的实例，y1 是指向 x 的引用
    let y1 = &x;
    let y2 = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);
    assert_eq!(5, *y2);

    // 变量被 drop 的顺序与其被创建的顺序相反，即，先删除 d，再删除 c
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // rust 不允许人工调用 Drop trait 的 drop 方法，因为 rust 在销毁资源时还会再次调用 drop，导致
    // 资源的重复释放。
    // 故：c.drop() 会导致 error
    // 如果想要提前释放资源，如释放锁，需要调用 std::mem::drop
    // 该方法已经被加入到 prelude 中，不需要显示引入
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

#[cfg(test)]
mod test {
    use super::*;

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    #[test]
    fn test_mybox_deref_coercions() {
        // MyBox 实现了 Deref，所以 rust 可以将 &MyBox<String> 转为 &String；
        // 标准库为 String 实现了 Deref，返回一个 string slice，所以 rust 可以调用 deref 将
        // &String 转为 &str，这样就满足了 fn hello(&str) 的要求
        let m = MyBox::new(String::from("Rust"));
        hello(&m);

        // 如果没有 deref coercion 机制，则上述代码要写成
        // (*m) 解引用 MyBox<String> 得到 String，& 和 [..] 将 String 转为 &str
        hello(&(*m)[..]);
    }
}