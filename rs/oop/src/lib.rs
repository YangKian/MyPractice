pub trait Draw {
    fn draw(&self);
}

// This vector is of type Box<dyn Draw>, which is a trait object;
// it’s a stand-in for any type inside a Box that implements the Draw trait.
// 使用 trait object 而不是泛型，是因为如果使用泛型，则在用一时间，泛型只能代替同一种实体类型
// 而 trait object 允许多种实体类型填充到 trait object 中
//
// 这种用法类似于 go 中的 interface 做参数
// 存在的问题：存在运行时性能开销，同时会阻止一些内联优化
//
// trait object 的对象安全问题：
// 一个 trait 满足对象安全的条件是，定义在 trait 中的所有的方法满足以下性质：
//  - 返回类型不是 self
//  - 没有泛型类型参数
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct ScreenGeneric<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenGeneric<T>
where T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self)
    }
}