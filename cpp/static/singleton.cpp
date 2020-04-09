#include <iostream>

class Singleton{
public:
    static Singleton& Get() { // 类内静态方法获取类的实例，返回一个引用，永远指向创建出来的单例
        static Singleton instance; // 函数内部静态变量，该变量创建出来以后不会随着函数栈空间退出被销毁
        return instance;
    }

    void Hello()
    {
        std::cout << "This is a Singleton class" << std::endl;
    };
};

void access_Singleton()
{
    Singleton::Get().Hello();
}