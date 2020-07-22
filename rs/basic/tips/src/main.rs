fn foo(input: Option<i32>) -> Option<i32> {
    // match input {
    //     Some(v) if v >= 0 => Some(v),
    //     None => None,
    //     _ => None,
    // }

    // input.and_then(|v| {
    //     if v >= 0 {
    //         Some(v)
    //     } else {
    //         None
    //     }
    // })

    input.filter(|i| *i >= 0)
}

fn bar(input: Option<i32>) -> Result<i32, String> {
    input.ok_or(String::from("wrong"))
}

fn ping_all(foos: &[Foo]) {
    foos.iter().for_each(|f| f.ping());
}

/// 输出：0:0, 1:1, 2:2, 3:3, 4:42
fn foo1() {
    let vec = vec![0, 1, 2, 3];
    for (i, v) in vec.iter()
        .chain(Some(42).iter())
        .enumerate() {
        println!("{}:{}", i, v);
    }
}

fn main() {
    if let Some(res) = foo(Option::from(-1)) {
        println!("{:?}", res);
    } else {
        println!("None")
    }

}
