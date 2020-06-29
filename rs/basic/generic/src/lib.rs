// trait：声明了特定类型所拥有的能力，且该能力可以与其他类型共享
// trait 类似于其他语言中的 interface
// A type’s behavior consists of the methods we can call on that type. Different
// types share the same behavior if we can call the same methods on all of those
// types. Trait definitions are a way to group method signatures together to define
// a set of behaviors necessary to accomplish some purpose.

use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String; // 注意结尾的分号不能漏
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 默认实现
// 默认实现中可以调用相同 trait 中的其他方法
pub trait SummaryWithDefaultImplementation {
    // 实现该 trait 的类型可以重载默认实现的方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 因为 summarize 已经有默认实现了，所以只需要实现 summarize_author 方法即可
impl SummaryWithDefaultImplementation for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用 traits 作为参数，任意实现了该 trait 的类型都可以作为函数的参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}
// 上面的 impl trait 语法实际上是语法糖，其真实语法是 trait bound 语法：
// pub fn notify<T: Summary> (item: &T) {...}
// 原始的 trait bound 语法提供了更多的信息，如：
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T){}
// 第一个函数使用了 impl trait 语法糖，则不要求 item1 和 item2 有相同的类型，只要他们都实现了
// Summary trait 即可
// 第二个函数使用了 trait bound, 则指明了，item1 和 item2 必须是相同类型 T

// 实现多个 trait
// 使用 Impl 语法糖
pub fn multi_trait_function_with_impl(item: &(impl Summary + Display)) {}
// 使用 trait bound
pub fn multi_trait_function_without_impl<T:Summary + Display>(item: &T) {}
// 使用 where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Summary + Clone,
          U: Clone + SummaryWithDefaultImplementation
{

}

// 使用 trait 作为返回值类型
// 注意：impl trait 作为返回类型，只能用于返回单一类型的情况，如果有两个不同的类型实现
// 了该 trait，想要通过 if ... else 等方式返回不同类型是不允许的
fn returns_summarizble() -> impl Summary {}