
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddr) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Message {
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    /**
     * rust 中没有 None 值，可以使用 Option
     * 标准库内置的枚举类型：
     * enum Option<T> {
     *      Some(T),
     *      None,
     * }
    */
    let some_number = Some(5);
    let some_string = Some("a string");
    // 注意 let absent_number = None; 会报错
    let absent_number: Option<i32> = None;

    // 注意 T 和 Option<T> 是不同的类型，以下代码会报错：
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let messages = [
        Message::Move{ x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit
    ];

    for message in &messages {
        message.call();
    }
}

// 使用 match 时必须列举出所有的可能，否则会报错
fn match_some_case() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        7 => println!("seven"),
        _ => (), // 如果只关心其中的部分结果，则使用 _ 去匹配其他任意未列出的值
    }
}

// 如果只想匹配一个模式，使用 if let，可以加上一个 else 子句
fn match_one() {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}