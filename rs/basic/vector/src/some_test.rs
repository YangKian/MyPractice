#![feature(const_generics)]

use smallvec::{SmallVec, smallvec};

#[test]
fn test() {
    // 在栈上分配内存
    let x = 32;
    println!("addr of x = {:?}", (&x as *const _));

    let mut v: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4];
    println!("addr of v's contents = {:?}", v.as_ptr());

    // 在堆上分配内存
    let heap_v = vec![1, 2, 3, 4];
    println!("addr of heap_v's contents = {:?}", heap_v.as_ptr());

    // 扩容，导致在堆上分配内存
    v.push(5);
    println!("addr of v's contents = {:?}", v.as_ptr());

    println!("size of usize：{:?}", std::mem::size_of::<usize>());
    dbg!(std::mem::size_of_val(&heap_v));
}

#[test]
fn test_dbg() {
    let x = 32;
    let mut v: SmallVec<[i32; 4]> = smallvec![dbg!(x), 2, 3, 4];
}

struct MyArray<const N: usize> {
    v: [u32; N],
}

impl<const N: usize> MyArray<N> {
    fn print_n(&self) {
        println!("N = {}", N);
    }
}

#[test]
fn test1() {
    let v = MyArray { v:[0u32, 1u32] };
    v.print_n();
}