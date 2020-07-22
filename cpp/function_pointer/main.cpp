#include <iostream>
#include <vector>

void HelloWorld() {
    std::cout << "Hello, World!" << std::endl;
}

void HelloWithParam(int a) {
    std::cout << "Hello, World, value: " << a << std::endl;
}

void PrintValue(int value) {
    std::cout << "Value: " << value << std::endl;
}

void ForEach(const std::vector<int>& values, void(*func)(int)) {
    for (auto value: values) {
        func(value);
    }
}

int main() {
    // 函数指针：void(*自定义的变量名)(参数类型)
    void(*myprint)() = HelloWorld;
    myprint();
    myprint();

    // 使用 typedef 定义别名
    typedef void(*HelloWorldFunction)(int);
    HelloWorldFunction function = HelloWithParam;
    function(1);

    std::vector<int> values = {1, 2, 3, 4, 5};
    ForEach(values, PrintValue);
}
