struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 只能将整个对象设置为可变的，不能单独设置某个成员可变
    // 初始化时必须显示赋值所有成员对象
    let mut user1 = User {
        email: String::from("someone@ex.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@ex.com");

    // Creating Instances From Other Instances With Struct Update Syntax
    // 注意：使用该方法会导致所有权的转移
    let user2 = User {
        email: String::from("cde@ex.com"),
        username: String::from("ddd"),
        // syntax .. specifies that the remaining fields not explicitly
        // set should have the same value as the fields in the given instance.
        ..user1 // 注意结尾没有逗号
    };
    let user3 = User{ ..user2 };

    // 元组结构体：只包含 field 的类型，不包含成员名
    // 使用与元组相同的 .index 的方式访问成员变量
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 单元结构体，其实例就是它自身，不管创建多少实例，编译器都会把它们优化为同一个
    // 单元结构体也不会占用实际的内存空间，是一个零大小类型
    struct Unit;

}

fn build_user(email: String, username: String) -> User {
    User{
        email, // 参数名和结构体 field 名相同时，可以直接使用参数名，省略 field 名
        username,
        active: true,
        sign_in_count: 1,
    }
}