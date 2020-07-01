use std::borrow::Cow;

pub mod rc;
pub mod refcell;
pub mod cell;

// Cow：Copy on write
fn escape(s: &str) -> Cow<str> {
    // escape函数的实现为：
    // 如果字符串中包含 '，则将其修改为 \'
    // 如果字符串中包含 '', 则将其修改为 \''
    // 否则直接返回
    let already_escaped;
    if s.contains("'") || s.contains("''") {
        already_escaped = false;
    } else {
        already_escaped = true;
    }

    if already_escaped {
        Cow::Borrowed(s) // 不会导致内存分配
    } else {
        let mut string = s.to_string();
        // do something to string (add \)
        Cow::Owned(string) // 这里才分配内存
    }
}