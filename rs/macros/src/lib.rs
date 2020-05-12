/**
 * 宏 VS 函数：
 *  - 宏在编译时展开，函数在运行时才被调用
 *  - 宏可以接受可变数量的参数，函数必须指定参数数量
*/

// declarative macros：
// #[macro_export] 注解用于导出宏，不添加该注解，则无法将宏引入到 scope 中
#[macro_export]
// macro_rules! 开启宏定义
macro_rules! vec {
    ( $( $x:expr ),* ) => { // 引起一个分支，如果正确匹配到该分支，则执行代码块
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macros：
// Procedural macros accept some code as an input, operate on that code,
// and produce some code as an output rather than matching against patterns
// and replacing the code with other code as declarative macros do.
// 有三类 procedural macros： (custom derive, attribute-like, and function-like)

