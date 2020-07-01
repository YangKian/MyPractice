/**
    unsafe Rust 提供了以下 5 种能力：
     - 解引用裸指针
     - 调用 unsafe 函数或方法
     - 访问或者修改一个可变的静态变量
     - 实现 unsafe trait
     - 访问 unions
*/

/**
    裸指针 raw pointer:
     - 有不可变和可变两种：*const T and *mut T
       *号不是解引用运算符，是类型的一部分
     - immutable means that the pointer can't be directly assigned to after being dereferenced.
    裸指针的特点：
     - 允许忽略 borrowing rules，同时持有可变和不可变指针，或者指向相同位置的多个可变指针
     - 不保证指向的内存是有效的
     - 允许空指针
     - 没有实现任何自动清理功能
    可以在 safe code 中创建裸指针，但是不能再 unsafe block 之外解引用裸指针
*/

fn main() {
    let mut num = 5;
    // 通过引用创建裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 只能在 unsafe block 中解引用并访问裸指针的数据
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
