/**
 * slice 没有 ownership
 * slice 让你可以引用集合中一段连续的元素，而不需要引用整个集合
*/

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..];
    let slice = &s[..];
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
