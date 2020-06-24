fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 注意：迭代器的执行是 lazy 的，所以如果这里不调用 .collect()，只调用 .map() 的话，
    // 实际上并没有执行迭代器，会报错：unused `std::iter::Map` that must be used
    // 注意要通过注解标明 v2 的类型
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// 标准库中 Iterator trait 的实现
// trait Iterator {
//     // type Item 和 Self::Item 定义了一个该 trait 的关联类型，Item 类型会是
//     // 该 trait 的返回类型
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     //......
// }