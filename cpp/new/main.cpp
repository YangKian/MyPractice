#include <iostream>

using string = std::string;

class Entity {
private:
    string m_Name;
public:
    // 构造函数的另一种写法：Constructor Initializer List
    // 可以在 {} 中添加其他初始化逻辑
    Entity() : m_Name("Unknown") {}
    Entity(const string& name) : m_Name(name) {}

    const string& GetName() const { return m_Name; }
};

int main() {

    // 调用默认构造器
    Entity e;
    Entity e_1 = Entity();

    // 使用参数进行初始化
    Entity e2("Tome");
    Entity e2_1 = Entity("Tome");

    // new 关键字：在堆上分配内存，并返回一个指针，同时还会调用对应的构造器
    Entity* e3 = new Entity();
    Entity* e4 = new Entity[50];

    // malloc 分配内存是 C 中的用法，与 new 的区别在于，malloc 只分配内存，不会调用构造器
    Entity* e5 = (Entity*)malloc(sizeof(Entity));
    // malloc 分配的内存要使用 free 来释放
    free(e5);

    // 使用 new 关键字分配的空间要使用 delete 进行回收
    delete e3;
    delete[] e4;
}
