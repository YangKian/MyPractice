mod function_and_parameter_ownership;

/**
 * String 的底层数据结构由三部分组成：
 *  - ptr：a pointer to the memory that holds the contents of the string
 *  - len：how much memory, in bytes, the contents of the String is currently using.
 *  - capacity：the total amount of memory, in bytes, that the String has received
        from the operating system.
 * 整个数据结构存储在栈中，而 String 的字面量值存在堆中
 *
 * rust 不会自动创建深拷贝，因此任何自动拷贝在运行时开销中都很廉价
 *
 * 编译时已知大小的数据被存储在栈上
*/

fn main() {

    let mut s = String::from("hello"); // 使用字面量来创建 string
    s.push_str(", world!"); // 追加一个字面量到 string 中
    println!("{}", s);

    /** Ways Variables and Data Interact: Move */
    let s1 = String::from("hello");
    // 通过赋值语句将 s1 赋值给 s2，实际拷贝的是 string 的数据结构，而不是堆上的数据
    // 同时，Rust 认为 s1 已经不再有效，当 s1 退出 scope 时，不会发生内存释放
    // 该行为被称为 move
    let s2 = s1;
    // println!("{}, world!", s1); 会编译报错：error[E0382]: borrow of moved value: `s1`

    /** Ways Variables and Data Interact: Clone */
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    println!("s1 = {}, s2 = {}", s1, s2);

    /** Stack-Only Data: Copy */
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    #[derive(Debug)]
    struct T {
        name: String,
        v: i32,
    }

    let mut p = T { name: "Tome".to_string(), v: 1 };
    // 对于结构体，转移其中部分字段的所有权后，再调用整个结构体会报错
    let name = p.name;
    // println!("{:?}", p);
    // 但是可以调用未转移所有权的字段
    // println!("{:?}", p.v);
    // 在重新恢复确实所有权字段的所有权后，又可以继续调用整个结构体
    p.name = "Jerry".to_string();
    println!("{:?}", p);

}
