use std::collections::HashMap;
/**
 * hashMap 和 vector 一样在堆上分配内存
 * hashMap 也只能存储同一类型的键值对
*/
fn main() {
    // 使用插入键值对的方式创建哈希表
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 使用迭代器、vector 和 colloct 方法创建哈希表
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = // 注意这里类型注解 HashMap<_, _> 不能少
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // 所有权的变更
    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values, as demonstrated
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 查
    // 注意.get()的参数是引用，返回结果是 Option<&V>
    let score = scores.get(&String::from("Blue"));

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 改
    // 覆写旧值
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 25);
    println!("{:?}", scores);
    // 只在键没有被赋值时插入值，即保留旧值
    // or_insert()：如果值存在，则返回该值的可变类型的引用，值不存在则为该键插入新的值
    scores.entry(String::from("Yellow")).or_insert(40);
    scores.entry(String::from("Black")).or_insert(40);
    println!("{:?}", scores);
    // 在旧值的基础上更新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() { // 统计句子中单词出现的次数并存入哈希表
        let count = map.entry(word).or_insert(0);
        *count += 1; // 注意解引用
    }
    println!("{:?}", map);
}
