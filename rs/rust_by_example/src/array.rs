#[cfg(test)]
mod test {

    #[test]
    fn array_test() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // 所有元素初始化为同一个值 0
        let ys: [i32; 500] = [0; 500]; // 注意是分号间隔
    }
}