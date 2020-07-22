fn main() {
    //========= 遍历语句的几种写法 ===============
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // 在 .iter() 生成的迭代器上调用 for，得到的是值的引用
    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Still can use v1: {:?}", v1);

    // for 语法实际上是语法糖，本质上是转换为迭代器，然后调用 next 方法
    let v2 = vec![4, 5, 6];
    // 调用 for 会获取值的所有权，调用过后原数组不可用
    for val in v2 {
        println!("Got value {} from v2", val);
    }

    let v3 = vec![7, 8, 9];
    let mut iter = v3.into_iter();
    // .into_iter() 迭代器调用 next 返回的值也带有所有权
    while let Some(value) = iter.next() {
        println!("Got value {} from v3", value)
    }

    let v4 = vec![10, 11, 12];
    let mut idx = 0;
    while let Some(value) = v4.get(idx) {
        idx += 1;
        println!("Got value {} from v4", value)
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