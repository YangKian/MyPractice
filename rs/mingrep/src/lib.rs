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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    //  using () like this is the idiomatic way to indicate that we’re calling run for
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