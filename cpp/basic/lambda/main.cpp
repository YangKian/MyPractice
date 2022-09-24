#include <iostream>
#include <vector>
#include <functional>
#include <algorithm>

void ForEach(const std::vector<int>& values, void(*func)(int)) {
    for (auto value: values) {
        func(value);
    }
}

void ForEachClosure(const std::vector<int>& values, const std::function<void(int)>& func) {
    for (auto value: values) {
        func(value);
    }
}

void modify(const std::function<void()>& func) {
    func();
}

int main() {
    std::vector<int> values = {1, 5, 4, 2, 3};

    auto lambda = [](int value) { std::cout << "Value: " << value << std::endl; };

    ForEach(values, lambda);

    int a = 5;
    // lambda 函数中使用了外部变量，要设置变量传入的方式
    // lambda 表达式中，[]内填入的内容就是对外部变量捕获的形式
    // [=]：所有变量按值拷贝的方式传递
    // [&]：所有变量按引用的方式传递
    // [a, &b]：通过独立变量传入，a 表示按值拷贝的方式传入 a，&b 表示按引用的方式传入 b
    // []：表示不需要捕获外界变量
    auto closure = [=](int value) { std::cout << "Value: " << a << std::endl; };
    ForEachClosure(values, closure);

    // copy 语义下不能修改被捕获的值，如果要修改，要加上 mutable
    auto closure_with_mutable = [=]() mutable { a = 2; std::cout << "Value: " << a << std::endl; };
    modify(closure_with_mutable);
    std::cout << "a = " << a << std::endl;
    auto closure_with_modify = [&]() { a = 3; std::cout << "Value: " << a << std::endl; };
    modify(closure_with_modify);
    std::cout << "a = " << a << std::endl;

    // 使用 algorithm 包中的 find_if 函数，找到迭代器中满足条件的第一个值
    auto first = std::find_if(values.begin(), values.end(), [](int value) { return value > 3; });
    std::cout << *first << std::endl;
}