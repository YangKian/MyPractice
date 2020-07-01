use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//
//         // 使用 clone() 导致浪费性能
//         let query = args[1].clone();
//         let filename = args[2].clone();
//
//         // 检查环境变量
//         // using the is_err method on the Result to check whether it’s an error and therefore unset
//         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//
//         Ok(Config{ query, filename, case_sensitive })
//     }
// }

// 优化
impl Config {
    // 直接将 env::args 返回的迭代器的 ownership 传递给 Config::new()
    // 注意返回值 Result 枚举，Result<T, E> 实际上是泛型实现，Err 对应的结果可以自定义，返回一个 &'static str
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        // 检查环境变量
        // using the is_err method on the Result to check whether it’s an error and therefore unset
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{ query, filename, case_sensitive })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be. This gives
// us flexibility to return error values that may be of different types in different error
// cases. The dyn keyword is short for “dynamic.”
// Error trait 定义在标准库中，所以需要引入：use::std::error::Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 运算符，作用在 Result 上，如果结果是 error，则直接退出不会继续向下执行
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // using () like this is the idiomatic way to indicate that we’re calling run for
    // its side effects only; it doesn’t return a value we need.
    Ok(())
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// 优化
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // .lines() 生成一个迭代器，按照 \n 或者 \r\n 来划分字符串
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}