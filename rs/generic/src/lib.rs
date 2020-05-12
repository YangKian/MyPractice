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
pub trait SummaryWithDefaultImplementation {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl SummaryWithDefaultImplementation for Tweet {}

// 使用 traits 作为参数
fn notify(item: impl Summary) {}

// 实现多个 trait
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Summary + Clone,
          U: Clone + SummaryWithDefaultImplementation
{

}

// 使用 trait 作为返回值类型
fn returns_summarizble() -> impl Summary {}