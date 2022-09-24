#include <iostream>
#include "assert.h"

class Base {
public:
    Base() { }
    virtual ~Base() { }
};

class Derived : public Base {
public:
    Derived() { };
    ~Derived() { };
};

class AnotherClass : public Base {
public:
    AnotherClass() { };
    ~AnotherClass() { };
};

struct Entity {
    int x, y;
};

int main() {
//    static_cast
//    reinterpret_cast
//    dynamic_cast
//    const_cast

    Entity e = { 5, 8 };
    int *position = (int*)&e;
    assert(position[0] == 5);
    assert(position[1] == 8);

    double value = 5.25;
    // 执行静态检查，发现不合法
    // double s = static_cast<AnotherClass *>(&value);

    // 可以正常执行
    AnotherClass *s = reinterpret_cast<AnotherClass *>(&value);
    std::cout << s << std::endl;

    // dynamic_cast 用于验证父类的实例是否是某个子类，如果是，则返回将父类转型为子类后
    // 的指针，如果不是则返回空指针。
    // 是通过存储 RTTI（runtime type information）来实现的，因此有运行时开销

    // derived 有两个类型：Derived 和 Base
    Derived* derived = new Derived();
    Base* base = derived; // 子类可以直接转型为父类，但是父类不知道子类的具体信息

    // 因为 base 是子类 Derived 的实例，因此转型为 AnotherClass 得到的是空指针，说明转型失败
    AnotherClass* ac = dynamic_cast<AnotherClass *>(base);
    if (ac == nullptr) {
        std::cout << "ac is not an instance of AnotherClass." << std::endl;
    }

    return 0;
}
