/**
    rust 中的宏分为：
     - declarative macros 声明宏，使用 macro_rules!
     - procedural macros 过程宏，有三种
       - 自定义 #[derive] 宏，在结构体和枚举类型上通过 derive 属性添加指定的代码
       - 类属性（Attribute-like）宏：用于任意项的自定义属性
       - 类函数宏：look like function calls but operate on the tokens
          specified as their argument
     - 声明宏主要做的是模式匹配和替换，过程宏接收一段代码，操作并执行代码产生输出，更像函数

     函数签名必须声明函数参数个数和类型。而宏可以接受可变参数
     在调用宏之前必须先定义并将其引入作用域
*/

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
