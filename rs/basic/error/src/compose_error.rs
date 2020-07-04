use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::fmt;
use std::{num, error};

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

// 通过实现 Error trait，可以提供对错误的具体描述
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

// 该函数可能会产生两种不同类型的错误，为了统一返回类型，需要做额外的封装
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    // // map_err() 作用在 Result 上，用来处理 Err 的情况
    // let mut file = File::open(file_path).map_err(CliError::Io)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(CliError::Io)?;
    // let n = contents.trim().parse::<i32>().map_err(CliError::Parse)?;
    // Ok(2 * n)

    // 通过实现 From trait 来完成 error 的类型转换
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(n * 2)
}

// 由于 Box<Error + Send + Sync> 已经实现了 From trait，故可以用
// Result<T, Box<dyn Error + Send + Sync>> 来封装错误
fn file_double_with_box<P: AsRef<Path>>(file_path: P)
    -> Result<i32, Box<dyn error::Error + Send + Sync>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(n * 2)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_openfile_error() {
        match file_double("a") {
            Err(e) => {
                println!("Error: {}", e);
                println!("Caused by: {}", e.source().unwrap());
            }
            _ => {} // skip it
        }
    }

    #[test]
    fn test_parse_error() {
        match file_double("a.txt") {
            Err(e) => {
                println!("Error: {}", e);
                println!("Caused by: {}", e.source().unwrap());
            }
            _ => {} // skip it
        }
    }

    #[test]
    fn test_box_openfile_error() {
        match file_double_with_box("a") {
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {} // skip it
        }
    }

    #[test]
    fn test_box_parse_error() {
        match file_double_with_box("a.txt") {
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {} // skip it
        }
    }
}