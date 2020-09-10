// TODO: 基于 https://www.twitch.tv/videos/716249625
// 本意是检查 enum 的大小，没看完，留下代码展示某些用法

#[derive(Debug)]
struct A {
    a: String,
    b: i32,
}

#[derive(Debug)]
struct B {
    arr: [u32; 32],
}

#[derive(Debug)]
enum Surprise {
    A(A),
    B(B),
}

fn show_first_four_bytes<T>(name: &str, t: &T) {
    let addr = t as *const _ as *const u8;
    let slice = unsafe { std::slice::from_raw_parts(addr, 4) };
    println!("{}'s first few bytes: {:?}", name, slice);
}

fn show_last_four_bytes<T>(name: &str, t: &T) {
    let addr = t as *const _ as *const u8;
    let len = std::mem::size_of_val(t);
    let slice = unsafe { std::slice::from_raw_parts(addr, len) };
    let last_4 = &slice[len - 4..];
    println!("{}'s first few bytes: {:?}", name, last_4);
}

#[test]
fn test() {
    let surprise1 = Surprise::A( A {
        a: "hello there".into(),
        b: 47,
    });

    let surprise2 = Surprise::B( B {
        arr: Default::default(),
    });

    // 注意要用引用，dbg! 会转移所有权
    // 即：let a = dbg!(surprise)； 是合法的
    dbg!(&surprise1);
    println!("size of surprise = {}", std::mem::size_of_val(&surprise1)); // 136
    println!("size of surprise = {}", std::mem::size_of_val(&surprise2)); // 136
    println!("size of A = {}", std::mem::size_of::<A>()); // 32
    println!("size of B = {}", std::mem::size_of::<B>()); // 128

    show_first_four_bytes("surprise1", &surprise1);
    show_first_four_bytes("surprise2", &surprise2);

    show_last_four_bytes("surprise1", &surprise1);
    show_last_four_bytes("surprise2", &surprise2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn t() {
        #[repr(C)]
        struct A {
            first: u8,
            second: u8,
            forth: u32,
            third: u8,
        }

        assert_eq!(12, std::mem::size_of::<A>());

        #[repr(C)]
        struct B {
            first: u8,
            second: u8,
            third: u32,
            forth: u16,
        }
        assert_eq!(12, std::mem::size_of::<B>());

        struct C(u16, u8, u16);
        assert_eq!(6, std::mem::size_of::<C>());

        struct D{}
        assert_eq!(0, std::mem::size_of::<D>());

        assert_eq!(std::mem::size_of::<Box<i32>>(), std::mem::size_of::<usize>())
    }
}

