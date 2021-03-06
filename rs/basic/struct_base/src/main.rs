mod basic;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // self 的三种形式分别代表不同的 ownership
    // self - Value
    // &self - shared reference
    // &mut self - mutable reference
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数，常用于类的构造函数，使用 :: 语法来调用关联函数
    // 关联函数不是结构体的方法，它与结构体相关联，不作用于具体的结构体实例
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 结构体的解构，解构不会导致所有权的转移
    // 注意：解构时成员名不能变
    let Rectangle{ width, height } = rect1;
    println!("width: {}, height: {}", width, height);
    let Rectangle{ width, ..} = rect1; // 使用 .. 来忽略掉不需要的值
    println!("width: {}", width);
}
