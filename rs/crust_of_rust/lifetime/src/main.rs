// 结构体中的成员可以是引用，此时需要加上 lifetime 注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime 注解的缺省
// rust 在编译时会依照以下三条规则进行检查
//  - 每个引用类型的参数都有自己的 lifetime 参数
//    fn first_word(s: &str) -> &str 等效于 fn first_word<'a>(s: &'a str) -> &'a str
//  - 如果存在一个指定的输入 lifetime 参数，则所有输出 lifetime 参数都等于该输入参数
//  - 如果有多个输入 lifetime 参数，但是其中一个是 &self 或者 &mut self（即该函数是一个方法），那么
//    所有的输出 lifetime 参数都等于 self 的 lifetime 参数
// 如果经过以上三条规则的检查后，编译器依旧无法确定返回值的 lifetime，则报错
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 实现了规则三
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// lifetime syntax is about connecting the lifetimes of various parameters and return
// values of functions.
// x 和 y 都是引用类型，不确定最终返回的到底是 x 还是 y，即无法确定返回值到底应该存在多久
// 所以需要引入 lifetime
// 在这个例子中，由于 x 和 y 都有相同的 lifetime 注解，则 'a 指的是两者中较小生命期的值
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 注意 lifetime 注解写在 & 符号后面
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 静态 lifetime：'static
// 意味着该引用在整个程序运行期间都是有效的
// 所有的字符串字面量都拥有 'static lifetime
// let s: &'static str = "I have a static lifetime.";

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 错误的写法，因为 result 的 lifetime 与 string1 和 string2 中较小的那个相同，即与 string2
    // 相同，调用 println 时 string2 已经超出作用域，故 result 也不再有效
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}