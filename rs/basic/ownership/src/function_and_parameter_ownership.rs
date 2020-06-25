// rust 通过所有权系统来进行内存管理。
// 所有权系统制定了一系列的规则，编译器在编译时会检查这些规则
// 所有权系统的引入不会导致运行时速度变慢

// 所有权规则：
//  - Each value in Rust has a variable that's called its owner
//  - There can only be one owner at a time
//  - When the owner goes out of scope, the value will be dropped.

// rust 垃圾回收的方式类似于 RAII（Resource Acquisition Is Initialization），当
// 变量离开其作用域后，rust 会调用一个特殊的函数 drop，该函数会完成资源的释放
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // 传入参数，其所有权也会被 move，即 s2 非法

    let (s4, len) = calculate_length(s1); // 使用元组来获取多个返回值

    // 使用引用来传递参数，不会发生所有权变更，使用引用作为函数参数的行为称为 borrowing
    // 如果在函数中尝试修改引用参数，会在编译时报错
    let len1 = calculate_length_with_reference(&s1);

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    // 限制：you can have only one mutable reference to a particular piece of
    // data in a particular scope
    // 以下代码会报错
    // let r1 = &mut s;
    // let r2 = &mut s;
    // 修正：使用 {} 创建一个单独的作用域
    { let r1 = &mut s; }
    let r2 = &mut s;

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; 报错：当有一个不可变引用时，不能再拥有可变引用

    // 注意：Note that a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used
    println!("{} and {}", r1, r2); // 在这之后r1和r2都不会再被使用，则其引用的作用域到此为止

    let r3 = &mut s; // 此时可以重新定义可变引用 r3

}

// Dangling Reference 会报错
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!


fn gives_ownership() -> String { // 返回值的所有权会被 move 给调用者
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
