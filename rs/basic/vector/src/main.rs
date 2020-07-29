fn main() {
    let v: Vec<i32> = Vec::new(); // 注意要加入类型注解，说明该 vector 中存储的是什么类型的元素
    let v = vec![1, 2, 3]; // 使用宏 vec! 来创建一个初始化的 Vec<i32>
    let v = vec![0; 20]; // 带有默认值的初始化，第一个元素是默认值，第二个元素是 vector 的长度

    // 增
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // drop：退出作用域后，vector 被释放，其内部存储的元素也会被清理
    {
        let v = vec![1, 2, 3, 4];
    }

    // 两种索引取值方法：
    //  - & 和 []：返回一个引用
    //  - .get()方法：返回一个 Option<&T>
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 可以显示声明类型
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100]; // 下标越界会 panic
    let does_not_exist = v.get(100); // 下标越界返回 None

    // ownership 问题：在相同的作用域中不能同时拥有可变引用和不可变引用
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow
    // v.push(6); error: mutable borrow

    // 迭代
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 注意解引用
    }

    // vector 只能存储相同类型的值
    // 使用枚举来存储不同变量
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
