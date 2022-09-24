#![feature(trace_macros)]
/**
声明宏类似于模式匹配，使用被匹配到的分支的代码进行替换。
每个分支可以有多个参数。参数以 $ 开头，接上 token 的类型：
 - item: 代表语言项，即组成一个 Rust 包的基本单位，如：模块、声明、函数定义
          类型定义、结构体定义、impl 实现等
 - block: 语句块，用花括号包裹
 - stmt：语句，一般是指以分号结尾的代码
 - pat: 模式
 - expr：表达式
 - ty: 类型
 - ident: 标识符
 - path: 路径，e.g. foo, ::std::mem::replace
 - meta: 元信息, e.g. the things that go inside #[...]
         and #![...] attributes
 - tt: single token tree
 - vis: 可见性
*/

macro_rules! add {
    ($a:expr)=>{
        $a
    };
    ($a:expr,$b:expr) => {{
        $a + $b
    }};
    ($a:expr,$b:expr,$typ:ty)=>{
        {
            $a as $typ + $b as $typ
        }
    };
    // 重复模式匹配：$(模式) sep rep
    // sep：参数分隔符，可以是逗号，分号，火箭符等，也可以省略
    //  比如：$a:ident, $b:tt，则
    //        $a($b) 实际可以匹配 funcName(arg)
    // rep：控制重复的次数，* 表示重复零次或多次，+ 表示至少重复1次
    ($a:expr,$($b:tt)*)=>{
        {
            $a + add!($($b)*)
        }
    };
}


macro_rules! hashmap {
    ($($key:expr => $value:expr),*$(,)*) => {
        {
            // 使用绝对路径，避免冲突
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

macro_rules! ok_or_return{
 // internal rule.
    // 内部宏以 @ 开头（非必须，惯用法），且宏名必须放到真正的匹配规则之前
    (@error $a:ident,$($b:tt)* )=>{
        {
        match $a($($b)*) {
            Ok(value)=>value,
            Err(err)=>{
                return Err(err);
            }
        }
        }
    };

// public rule can be called by the user.
    ($a:ident($($b:tt)*))=>{
        ok_or_return!(@error $a,$($b)*)
    };
}

fn some_work(i:i64,j:i64)->Result<(i64,i64),String>{
    if i+j>2 {
        Ok((i,j))
    } else {
        Err("error".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        println!("{}", add!(1, 2));
        println!("{}", add!(0, 2, u8));
        println!("{}", add!(1, 2, 3, 4));
    }

    #[test]
    fn test_hash_map() {
        // trace_macros!(false);
        let map = hashmap!(
            "a" => 1,
            "b" => 2,
        );
        println!("{:?}", map)
    }

    #[test]
    fn test_ok_or_return() -> Result<(), String> {
        /**
            ok_or_return!(@error some_work, 1, 4) => {
                match some_work(1,4) {
                    Ok(value) => value
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
         */
        println!("{:?}", ok_or_return!(some_work(1,4)));
        println!("{:?}", ok_or_return!(some_work(1,0)));
        Ok(())
    }
}
