
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // 注意，调用 next（）方法会改变迭代器的内部状态，所以要将 v1_iter 定义为 mutable 的
    // 调用 next() 获得的是不可变的引用
    // If we want to create an iterator that takes ownership of v1 and returns
    // owned values, we can call into_iter instead of iter.
    // If we want to iterate over mutable references, we can call iter_mut instead of iter.
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    println!("{:?}", v1); // [1,2,3]
    println!("{:?}", v1_iter) // Iter([])
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // sum 会获取 v1_iter 的所有权，所以 sum 之后 v1_iter 不能再使用
    assert_eq!(total, 6);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 实现迭代器，只需要实现 next 方法
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

//take the values produced by an instance of Counter, pair them with values
// produced by another Counter instance after skipping the first value, multiply
// each pair together, keep only those results that are divisible by 3, and
// add all the resulting values together
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        // zip() 生成了四个pair,最后一个pair（5，None）不会被生成
        // 因为当 zip 的任一一个输入迭代器生成 None 时，zip 返回 None
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // .iter()：Returns an iterator over the slice. 函数签名为：pub fn iter(&self) -> Iter<'_, T>
    //  - .iter() 获取的是引用，不会获取原 vec 的所有权
    // .into_iter()：Creates a consuming iterator，函数签名为：fn into_iter(self) -> IntoIter<T>
    //  - .into_iter() 会获取 vec 的所有权，该方法调用后 vec 不可用
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}