fn main() {
    // 元组：元组中的各元素类型可以不同
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 访问元组值的两种方式
    // 模式解构
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // 索引访问
    let five_hundred = tup.0;
    let one = tup.2;

    // 数组：数组中的各元素类型必须相同
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // 创建一个每个元素都相同的数组 => [3, 3, 3, 3, 3]

    // 数组下标越界在编译时不会报错，但是运行时会报错
    let first = a[0];
    let second = a[1];

    another_function(5);
}

// 函数的参数必须指明其类型
fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn expression() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 表达式不包含结束分号
    };

    println!("The value of y in expression is: {}", y);
}

// 使用 -> 来声明返回值
fn five() -> i32 {
    5
    // 注意这里不能加分号，因为该函数有返回值，所以是一个表达式，如果加了分号则该函数变成 statement，
    // statement 是不生成值的，故加了分号后，在其他地方调用 let a = five() 会报错
}

fn if_statement() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 是一个表达式，所以可以将其赋值给变量
    // if 的每个分支返回的值的类型必须相同
    let condition = true;
    let n = if condition { 5 } else { 6 }; // 注意两个 {} 中的值类型必须相同
    println!("The value of number is: {}", n);
}

// loop 是无限循环
fn loop_circle() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_circle() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_circle() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // this approach is error prone; we could cause the program to panic if the index
    // length is incorrect. It’s also slow, because the compiler adds runtime code to
    // perform the conditional check on every element on every iteration through the loop.
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // A Range, which is a type provided by the standard library that generates all numbers
    // in sequence starting from one number and ending before another number.
    for number in (1..4).rev() { // rev() 翻转迭代器的值
        println!("the value is: {}", number);
    }
}