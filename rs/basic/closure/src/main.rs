use std::thread;
use std::time::Duration;
use closure::{Fnc, FncMove};


// 通过创建一个结构体来持有闭包和其生成的值，这样在我们需要生成值时可以调用闭包，然后缓存结果，避免
// 后续使用时还要重复计算。
// 存在的问题：定义结构体，枚举等需要知道成员的类型，通过泛型和 trait bound 来实现
// Fn trait 由标准库提供，所有的闭包都至少实现了以下三个 trait 中的一个：Fn, FnMut, or FnOnce.
// Add types to the Fn trait bound to represent the types of the parameters
// and return values the closures must have to match this trait bound.
struct Cacher<T>
where T: Fn(u32) -> u32, // 注意这里 trait bound 的用法
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// TODO: 增加泛化能力
// struct Cacher<T, R>
//     where T: Fn(R) -> R, // 注意这里 trait bound 的用法
// {
//     calculation: T,
//     value: HashMap<R, R>,
// }

// impl<T, R> Cacher<T, R>
//     where
//         T: Fn(R) -> R,
//         R: Hash + Eq + PartialEq
// {
//     fn new(calculation: T) -> Cacher<T, R> {
//         Cacher {
//             calculation,
//             value: HashMap::new(),
//         }
//     }
//
//     fn value(&mut self, arg: &R) -> R {
//         let result = self.value.entry(*arg).or_insert((self.calculation)(arg));
//         result
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // we’re using a closure because we want to define the code to call at
    // one point, store that code, and call it at a later point;
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // 错误的用法：闭包定义中不需要显示声明参数和返回值的类型，编译器会自动进行推断，但是只要闭包调用一次之后，
    // 编译器会记录下经过推断得到的参数类型，下次再调用闭包时如果传入了不一样的类型就会报错
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    // 每个闭包的实例都有自己唯一的匿名类型，也就是说，尽管两个闭包的函数签名相同，他们的类型也有可能不同

    // 闭包捕获上下文环境变量
    //  Closures can capture values from their environment in three ways, which directly map to
    //  the three ways a function can take a parameter: taking ownership, borrowing mutably, and
    //  borrowing immutably. These are encoded in the three Fn traits as follows:
    //
    //     - FnOnce consumes the variables it captures from its enclosing scope, known as
    //       the closure’s environment. To consume the captured variables, the closure must
    //       take ownership of these variables and move them into the closure when it is
    //       defined. The Once part of the name represents the fact that the closure can’t
    //       take ownership of the same variables more than once, so it can be called only once.
    //     - FnMut can change the environment because it mutably borrows values.
    //     - Fn borrows values from the environment immutably.
    //
    //  rust 根据闭包如何使用环境变量来推断应该使用哪一种 trait
    //  All closures implement FnOnce because they can all be called at least once.
    //  Closures that don’t move the captured variables also implement FnMut, and
    //  closures that don’t need mutable access to the captured variables also implement Fn.

    let x = 4;
    let equal_to_x = |z| x == z; // 使用了 Fn trait，即不可变引用
    let y = 4;
    assert!(equal_to_x(y));

    // 以下代码会报错
    // fn equal_to_x(z: i32) -> bool {
    //    z == x
    // }
    // assert!(equal_to_x(y))
    // error[E0434]: can't capture dynamic environment in a fn item

    // 如果希望强制闭包获取环境变量的所有权，在参数列表前使用 move 关键字
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    // ===================================================

    let fnc = Fnc{ s: "test".to_string() };
    let f1 = fnc(1, 2);
    assert_eq!(f1, "test".to_string());
    // let f2 = fnc(2, 3); 不能再使用，因为此时 fnc 变量已经 move 到 f1

    let str = "test".to_string();
    let mut fnc_mut = FncMove{ s: &str };
    let f1 = fnc_mut(1, 2);
    assert_eq!(f1, "test".to_string());
    let f2 = fnc_mut(2, 3);
    assert_eq!(f2, "test".to_string());
}
