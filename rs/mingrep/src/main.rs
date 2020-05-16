use std::env; // 返回一个命令行参数的迭代器
use std::process;
use mingrep::Config;

// fn main() {
//     // env::args()返回一个命令行参数的迭代器，.collect() 方法可以将迭代器转换为 collection
//     let args: Vec<String> = env::args().collect();
//
//     // .unwrap_or_else： Result 的方法，允许我们定义错误处理的方式
//     // 如果 Result 的结果是 Err，该方法会调用闭包中的匿名函数
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
//
//     if let Err(e) = mingrep::run(config) {
//         eprintln!("Application error: {}", e);
//         process::exit(1);
//     };
// }

// 优化版
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mingrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}