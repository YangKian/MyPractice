use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop { // loop 创建一个无限循环
        println!("Please input your guess.");

        // :: 运算符表示 new 方法是 String 类型的一个关联函数，关联函数是针对类型实现的，类比为静态方法
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // 默认情况下引用是不可变的，因此要使用 &mut 来使其可变
            .expect("Failed to read line");

        // 类型转换
        // read_line() 方法会将输入的回车也写入到字符串中，调用 trim() 可以去除头尾的 whitespace
        // .parse()：将字符串解析为数字，因为数字有很多不同的类型，所以需要在变量声明的 let 语句中显示指定需要解析
        // 成哪种类型的值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value, here we're saying we want to match all Err values
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
