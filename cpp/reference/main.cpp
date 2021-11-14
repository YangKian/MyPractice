#include <iostream>

#define LOG(x) std::cout << x << std::endl

void Increment(int& num) {
    num++;
}

int main() {
    int a = 5;
    // ref 是 a 的引用，也就是 a 的别名，内存中只存在 a 一个变量，ref 变量只存在代码里。
    // 声明了引用的对象后，就不能再修改其引用的对象
    int& ref = a;

    Increment(a);
    LOG(a);

    return 0;
}
