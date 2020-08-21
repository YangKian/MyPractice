//! 三类 Fn trait 中， FnOnce 是基础，FnMut 派生自 FnOnce，Fn 派生自 FnMut
//! 相互转换的内在逻辑：call_once 内部调用 call_mut，因为所有权总是可以转换为 Mut Borrow,
//! Mut Borrow 也总是可以转换为 Borrow

//!    pub trait FnOnce<Args> {
//!         type Output;
//!         extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
//!     }

//!  pub trait FnMut<Args>: FnOnce<Args> {
//!     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
//!  }

//!  pub trait Fn<Args>: FnMut<Args> {
//!     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
//!  }

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