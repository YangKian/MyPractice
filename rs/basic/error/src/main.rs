mod question_mark_main;
mod error_test1;

use std::fs::File;
use std::io::ErrorKind;
//以下导入方法等价于：
// use std::io;
// use std::ip::Read;
use std::io::{self, Read};
use std::fs;

fn main() {
    let f = File::open("hello.txt");

    // open() 函数反回的结果是 Result 类型，Result 其实是枚举，有 Ok 和 Err 两个变量

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    //If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();

    // 可以自定义 panic 时输出的错误信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 提前返回
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ？运算符对每个 Result 类型进行判断，如果值为 Ok，则继续执行，如果值为 Err，则直接返回
// There is a difference between what the match expression do and the ? operator do: error values
// that have the ? operator called on them go through the from function, defined in the From trait
// in the standard library, which is used to convert errors from one type into another. When the ?
// operator calls the from function, the error type received is converted into the error type
// defined in the return type of the current function. This is useful when a function returns one
// error type to represent all the ways a function might fail, even if parts might fail for many
// different reasons. As long as each error type implements the from function to define how to
// convert itself to the returned error type, the ? operator takes care of the conversion
// automatically.
fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// fs::read_to_string() 方法一次性实现了打开文件，并读取内容
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 操作符只能用在返回值是 Result 或者 Option 或者其他实现了 std::ops::Try 的类型的情况下