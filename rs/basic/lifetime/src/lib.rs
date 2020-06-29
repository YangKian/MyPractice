mod main;

#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

// str -> [char] 字符数组，可能分配在堆上，也可能分配在栈上
// &str -> &[char]
// String -> Vec<char> vector，在堆上分配
// String -> &str （cheap -- AsRef）因为 String 的分配空间是确定的
// &str -> String （expensive -- Clone）因为 &str 不知道到底分配在哪里，只能拷贝到堆上构建 String

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// impl Delimiter for &str {
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            // 错误的写法：.find(|c| c == self)
            // 因为 .char_indices() 生成的是一个 CharIndices 类型的迭代器，
            // 即每个可迭代的元素都是一个 CharIndices 结构体，该结构体包含 （usize, char）两个对象
            // 此时 c 的类型是 (usize, char)，而 self 的类型是 char，无法比较
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // .as_mut()：Converts from `&mut Option<T>` to `Option<&mut T>`.
        let remainder /* &mut &'a str */ = self.remainder.as_mut()?; /* Option<&'a str> */
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    // 错误的写法：binary operation `==` cannot be applied to type `StrSplit<'_>`
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}