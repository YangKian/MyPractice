fn main() {
    // 元组：元组中的各元素类型可以不同
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let one = tup.2;

    // 数组：数组中的各元素类型必须相同
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // => [3, 3, 3, 3, 3]

    // 数组下标越界在编译时不会报错，但是运行时会报错
    let first = a[0];
    let second = a[1];
}
