use std::borrow::Cow;

fn remove_spaces(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}

#[test]
fn test_remove_spaces() {
    let s = remove_spaces("Herman"); // s is a Cow::Borrowed variant
    let len = s.len(); // immutable function call using Deref
    let owned: String = s.into_owned(); // memory is allocated for a new string

    let s = remove_spaces("Herman Radtke"); // s is a Cow::Owned variant
    let len = s.len(); // immutable function call using Deref
    let owned: String = s.into_owned(); // no new memory allocated as we already had a String
}