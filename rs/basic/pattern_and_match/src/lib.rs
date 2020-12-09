/**
    模式有两种形式：refutable 和 irrefutable
     - 能匹配任何传递的可能值的模式称为 irrefutable，如：let x = 5
     - 对某些可能的值进行匹配会失败的模式称为 refutable，如：if let Some(x) = value，
       因为 value 的值可能为 None 而不是 Some，此时 Some(x) 无法匹配
     函数参数、let 语句和 for 循环只接受 irrefutable 模式
     if let 和 while let 只接受 refutable 模式
     错误的使用模式会导致编译不通过，如：let Some(x) = value，if let x = 5
*/

#[cfg(test)]
mod test {

    // Named variables are irrefutable patterns that match any value
    // match 表达式中声明的变量会屏蔽外部的同名变量
    #[test]
    fn matching_named_variables() {
        // 最终输出:
        // Matched, y = 5
        // at the end: x = Some(5), y = 10
        let x = Some(5);
        let y = 10;
        match x { // 实际上是 match Some(5)
            Some(50) => println!("Got 50"),
            // 引入了一个新的变量 y，可以匹配任何 Some 中的值，且与外部定义的变量
            // y 不相关。满足匹配条件 Some(5)，执行该分支
            Some(y) => println!("Matched, y = {:?}", y),
            // 如果 x = None，则会执行到该默认分支
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    // 使用 | 运算符来匹配多个模式
    #[test]
    fn multiple_patterns() {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("other"),
        }
    }

    // ref 匹配
    #[test]
    fn ref_match() {
        fn f(x: &Option<String>) {
            match x {
                // &Some(s) => {println!("{:?}", s)}, // 报错：s 是 String 类型，没有实现 Copy，
                // &Some(s) 是共享引用，不允许转移 s 的所有权
                &Some(ref s) => {println!("{:?}", s)},
                // 或者直接 Some(s) => {...}，此时 s 的类型也是 &String，实际上是编译器自动实现了解引用和 ref 的插入
                None => {},
            }
        }

        let x = Some("hello".to_owned());
        f(&x);
    }

    #[test]
    fn matching_ranges_of_values() {
        let x = 5;
         match x {
             // 使用 ..= 语法来进行范围匹配，所有值在 [1,5] 之间的数都能被匹配到
             1..=5 => println!("one through five"),
             _ => println!("other"),
         }

        // 范围匹配只能用在数字类型和 char 类型的值上
        let x= 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("later ASCII letter"),
            _ => println!("other"),
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn destructure_structs() {
        let p = Point{ x: 0, y: 8 };
        let Point {x:a, y: b} = p;
        assert_eq!(0, a);
        assert_eq!(8, b);

        let Point {x, y} = p;
        assert_eq!(0, x);
        assert_eq!(8, y);

        let p = Point{ x: 0, y: 7 };
        match p {
            Point{ x, y: 0} => println!("On the x axis at {}", x),
            Point{ x:0, y} => println!("On the y axis at {}", y),
            Point{ x, y} => println!("On neither axis: ({},{})", x,y),
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn destructure_enum() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[test]
    fn destruct_nested_enum() {
        enum Message {
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            _ => (),
        }
    }

    #[test]
    fn ignore_parts_of_a_value() {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first,_,third,_,fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }

    #[test]
    fn extra_conditionals_with_match_guard() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
}