#include <iostream>

/**
 * 在类之外的其他地方使用 static 关键字：表示被 static 修饰的变量只属于定义该变量的编译单元，不会
 * 在链接阶段与其他部分进行链接
 *
 * 在类中使用 static 关键字修饰变量，表示该变量是一个类内的全局变量，该类的所有实例都共享该变量
 *
 * 在函数中使用 static 关键字修饰变量，表示该变量是函数的一个本地变量，用法类似于 python 中的 def func(a=[])，非常不建议使用这种用法
 */


int s_Variable = 5;

extern void Func(){};
void Do_not_use_like_this();
void access_Singleton();

struct Entity {
    static int x, y;
    int a, b;

    void print()
    {
        std::cout << x << ", " << y << std::endl;
    }

    /**
     * 注意以下用法会报错
     *  static void print()
     *  {
     *      std::cout << a << ", " << b << std::endl;
     *  }
     *  报错原因是因为静态方法没有类实例
     */
};

// 修正静态方法
static void print(Entity e)
{
    std::cout << e.a << ", " << e.b << std::endl;
}

// 注意不加下面两行会报错，因为后面的代码通过实例对类内静态变量进行了访问
int Entity::x;
int Entity::y;

int main() {

    // private_Variable = 7; 会报错，因为在static.cpp中声明的变量private_Variable被static修饰，表示这是一个私有变量

    Entity entity1;
    entity1.x = 2;
    entity1.y = 3;

    Entity entity2;
    entity2.x = 4;
    entity2.y = 7;

    // 正确的写法，通过命名空间访问静态变量
    Entity::x = 5;
    Entity::y = 9;

    entity1.print(); // x = 5, y = 9
    entity2.print(); // x = 5, y = 9

    // 函数内的static变量
    Do_not_use_like_this(); // 1
    Do_not_use_like_this(); // 2
    Do_not_use_like_this(); // 3
    Do_not_use_like_this(); // 4

    // 单例访问
    access_Singleton();
}
