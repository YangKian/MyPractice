//! 三类 Fn trait 中， FnOnce 是基础，FnMut 派生自 FnOnce，Fn 派生自 FnMut
//! 相互转换的内在逻辑：call_once 内部调用 call_mut，因为所有权总是可以转换为 Mut Borrow,
//! Mut Borrow 也总是可以转换为 Borrow

//!  pub trait FnOnce<Args> {
//!     type Output;
//!     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
//!  }
//!  call_once 的第一个参数 self 标明了所有权的转移，即被闭包捕获的对象，其所有权要转移到闭包中，也是 FnOnce 中
//!  Once 的含义：不能多次捕获相同变量，因为所有权发生了转移
//!  编译器会把 FnOnce 的闭包类型看成是函数指针
//!

//!  pub trait FnMut<Args>: FnOnce<Args> {
//!     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
//!  }

//!  pub trait Fn<Args>: FnMut<Args> {
//!     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
//!  }
//!
//! 注意，以上三个函数的第二个参数 Args 必须是元组类型，这是由 API 规定的

use std::ops::{Fn, FnMut, FnOnce};

pub struct Fnc {
    pub s: String,
}

impl FnOnce<(i32, i32)> for Fnc {
    type Output = String;

    extern "rust-call" fn call_once(self, args: (i32, i32)) -> Self::Output {
        println!("{}-{}", args.0, args.1);
        self.s
    }
}

pub struct FncMove<'a> {
    pub s: &'a String,
}

impl<'a> FnOnce<(i32, i32)> for FncMove<'a> {
    type Output = String;

    extern "rust-call" fn call_once(mut self, args: (i32, i32)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a> FnMut<(i32, i32)> for FncMove<'a> {
    extern "rust-call" fn call_mut(&mut self, args: (i32, i32)) -> Self::Output {
        println!("{}-{}", args.0, args.1);
        self.s.to_owned()
    }
}

//! 闭包自身实现的 trait
//!  - Sized：所有闭包默认实现
//!  - Copy/Clone：取决于环境变量是否实现 Copy 以及它如何被闭包使用
//!     1. 如果环境变量自身实现了 Copy, 闭包如果以可变借用的方式捕获环境变量，并对其修改，则闭包本身不会实现 Copy
//!        即：如果闭包会对环境变量进行修改，则其不会实现 Copy
//!     2. 如果环境比那里自身是 Move 语义，且闭包内捕获环境变量的操作涉及修改环境变量或者消耗环境变量，
//!        闭包自身不会实现 Copy
//!  - Sync/Send：
//!     1. 如果所有捕获变量均实现了 Sync，则闭包实现 Sync
//!     2. 如果环境变量都不是以 ”唯一不可变引用“ 方式捕获的，并且都实现了 Sync，则闭包实现 Send
//!     3. 如果环境变量是以 “唯一不可变引用”、“可变引用”、Copy 或 Move 所有权捕获的，则闭包实现 Send
//!