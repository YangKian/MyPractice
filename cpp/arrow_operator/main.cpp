#include <iostream>

/***
 * 箭头运算符的一个特殊用法：可以用来获取结构体中，成员变量在内存中的 offset
 */

struct Vector {
    float a, b, c;
};

int main() {
    auto offset_a = (intptr_t)&((Vector*) nullptr)->a;
    std::cout << offset_a << std::endl;
    auto offset_b = (intptr_t)&((Vector*) nullptr)->b;
    std::cout << offset_b << std::endl;
    auto offset_c = (intptr_t)&((Vector*) nullptr)->c;
    std::cout << offset_c << std::endl;
}
