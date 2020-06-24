struct Point<T, U> {
    x: T,
    y: U,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
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
