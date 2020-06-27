/**
 * slice 没有 ownership
 * slice 让你可以引用集合中一段连续的元素，而不需要引用整个集合
 * slice 的区间范围也是左闭右开
*/

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..];
    let slice = &s[..];

    // string 的字面量实际上是 slice，即 &str，所以不能修改 string 的字面量
    let s1 = "Hello world!";
    let res = first_word_with_str_and_string_param(&s);
    let res = first_word_with_str_and_string_param(&s1);

    let a = [1, 2, 3, 4, 5];
    // b 的类型是 &[i32]，存储了一个 slice 首元素的引用，以及 slice 的长度
    let b = &a[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 使用 &str 作为参数而不是 &String，这样既可以传入字面量，也可以传入 String
fn first_word_with_str_and_string_param(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}