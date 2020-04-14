#include <iostream>
#include <string>

// 模板只有在调用时，传入了具体的类型参数，才会编译
template <typename T>
void Print(T v) {
    std::cout << v << std::endl;
}

// 利用模板只在调用时才编译的特点，可以用来创建编译时大小未确定的数组 template <int N>
// 同时也可以指定泛型 template <typename T, int N>
template <typename T, int N>
class Array {
private:
    T m_size[N];
public:
    int GetSize() { return N; }
};

int main() {
    Print(1);
    Print<int>(5); // 可以通过 函数名<参数类型>() 的方式将参数类型传递给模板
    Print("Hello");
    Print(1.34f);


    Array<std::string, 50> array;
    std::cout << array.GetSize() << std::endl;
}
