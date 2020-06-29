// rust 中使用泛型不会带来运行时开销，因为在编译阶段，rust 通过 monomorphization 用实际类型填充了泛型参数
// 的部分

use std::fmt::Display;

struct Point<T, U> {
    x: T,
    y: U,
}

// the generic parameters T and U are declared after impl, because they go with the struct
// definition. The generic parameters V and W are declared after fn mixup, because they’re
// only relevant to the method.
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 只有实现了 PartialOrd 的类型才能比较大小
// when we made the largest function generic, it became possible for the list parameter
// to have types in it that don’t implement the Copy trait. Consequently, we wouldn’t be
// able to move the value out of list[0] and into the largest variable
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Another way we could implement largest is for the function to return a reference to
// a T value in the slice. If we change the return type to &T instead of T, thereby
// changing the body of the function to return a reference, we wouldn’t need the Clone
// or Copy trait bounds and we could avoid heap allocations.
fn largest1<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

// 所有的 Pair<T> 类型都实现了 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 只有那些为 T 类型实现了 PartialOrd 和 Display trait 的 Pair<T> 类型才会
// 实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 也可以为任意实现了其他 trait 的类型 T 有条件的实现某个 trait
// Implementations of a trait on any type that satisfies the trait bounds
// are called blanket implementations.
// 如：标准库中为任意实现了 Display trait 的类型实现了 ToString trait
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         ...
//     }
// }
// 由此，所有实现了 Display trait 的类型都可以调用 to_string() 方法

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point{x: 5, y: 6};
    let float = Point{x: 1.0, y: 3.0};
    let mix_type = Point{x: 9, y: 3.0};

    // Generic functions can be thought of as namespaces, containing an
    // infinity of functions with different concrete types.
    // 符号 ::<> 被称为 turbofish syntax
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"

    print_type_name(&integer)
}
