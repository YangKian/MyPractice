
#include <iostream>

static int s_Variable = 5; // 如果不加 static，会在链接阶段报错，因为 main 函数中也有一个相同的定义

static int private_Variable = 6;

static void Func()
{

}

void Do_not_use_like_this()
{
    static int i = 0;
    i++;
    std::cout << i << std::endl;
}