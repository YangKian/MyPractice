
// 声明宏
// #[macro_export] 注解指明，在宏定义被引入的 scope 内
// 该宏都是可用的
#[macro_export]
// 使用 macro_rules! 开始宏定义
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

// 过程宏
// use proc_macro;
// use proc_macro::TokenStream;
//
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}
