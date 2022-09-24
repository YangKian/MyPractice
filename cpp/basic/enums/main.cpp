#include <iostream>

/**
 *  枚举：默认从0开始，类型是 32-bit int
 */
enum Example
{
    A = 5, B, C
};

// 类型是 unsigned char 8-bit，注意类型只能选整形，不能用浮点型
enum Example_With_Type : unsigned char {
    a, b, c
};

int main()
{
    Example value = A;
    if (value == B)
    {
        //Do something
    }
    std::cout << A << std::endl;
    return 0;
}
