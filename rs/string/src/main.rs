/*
 * rust 中的 string 有两种：
 *  - 标准库中的 String
 *  - core language 中的 str(string slice)，通常是 borrowed form：&str
 * 两种 string 都使用 UTF-8 编码
*/

fn main() {
    let mut s = String::new();

    // 任意实现了 Display trait 的类型都可以使用 to_string() 方法
    let data = "inital conents".to_string();
    let s = String::from("initial contents");

    // 更新：String 的 size 和内容都是可变的
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str 方法的参数是 string slice，不会导致所有权转移
    print!("s2 is {}", s2);

    s.push('l'); // push 方法的参数是单个字符 char

    // 字符串拼接
    // 方法一：
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + 运算符连接两个字符串
    // 注意此处 s1 的 ownership 被 move，无法再使用
    // 这是由 fn add(self, s: &str) -> String {} 方法决定的，+ 运算符调用了 add 方法
    /**
        although let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
        this statement actually takes ownership of s1, appends a copy of the contents of s2,
        and then returns ownership of the result. In other words, it looks like it’s making
        a lot of copies but isn’t; the implementation is more efficient than copying.
    */
    let s3 = s1 + &s2;

    // 方法二：
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    /**
     * rust 的 string 不支持 indexing
     * String 其实是 Vec<u8> 的封装
    */
    // 可以通过 range 来取字符，但是要小心，因为编码的问题很可能取到无效值导致 crush

    let hello = "Здравствуйте";
    // let s = &hello[0..1]; 会crush
    let s = &hello[0..4];

    // 通过迭代的方式来获取字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
