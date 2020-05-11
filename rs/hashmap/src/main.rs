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
}
