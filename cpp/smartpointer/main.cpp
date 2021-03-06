#include <iostream>
#include <string>
#include <memory> // 智能指针包含在该头文件中

class Entity {
public:
    Entity()
    {
        std::cout << "Created Entity!" << std::endl;
    }
    ~Entity()
    {
        std::cout << "Destroyed Entity!" << std::endl;
    }

    static void Print() {
        std::cout << "Call Print!" << std::endl;
    }
};

// 自建智能指针类
class ScopedPtr {
private:
    Entity* m_Obj;
public:
    ScopedPtr(Entity* entity) : m_Obj(entity) {}

    ~ScopedPtr()
    {
        delete m_Obj;
    }

    // 重载运算符，注意这里加上第一个 const，则要将 Entity 对象中的 Print 方法改为只读
    const Entity* operator->() const {
        return m_Obj;
    }
};

int main() {
    {
        ScopedPtr entity = new Entity();
        // 想要让 ScopedPtr 的实例也可以调用 Entity 的方法，要重载 -> 运算符
        entity->Print();
    }

    {
        // 构建智能指针的方法一
        // 使用默认初始化的方式创建智能指针，表示可以指向一个 int 类型的对象，此时得到的是一个空指针
        std::shared_ptr<int> p1;

        // 构建智能指针的方法二
        // std::unique_ptr<Entity> entity(new Entity());

        // 构建智能指针的方法三，更推荐的做法，因为这种方式是 exception safety 的
        std::unique_ptr<Entity> entity = std::make_unique<Entity>();
        entity->Print();
    }

    // 智能指针：可以自动完成内存的释放
    // 问题：使用 unique_ptr 定义的是唯一指针，不能拷贝该指针，因为如果拷贝智能指针，意味着有两个不同的指针指向了
    // 同一块内存空间，当智能指针自动销毁时，拷贝的那个指针不会自动销毁，即该指针此时指向了一块被销毁的内存空间

    // 如果要拷贝指针，或者传递指针，则使用 shared_ptr
    // 使用 shared_ptr 构建的智能指针会分配在一个单独的控制块中，并且会对该指针的引用进行计数，只有等到计数为0，分配的内存才会被销毁
    // 不能使用 std::shared_ptr<Entity> e = entity(new Entity()) 的方式去创建 shared_ptr，因为这样等同于先在堆上创建一次，
    // 再在控制块中创建一次
    {
        std::shared_ptr<Entity> e1;
        {
            std::shared_ptr<Entity> entity = std::make_shared<Entity>();
            // 当使用 weak_ptr 来拷贝 shared_ptr 时，引用计数不会增加，即 weak_ptr 并不持有指针的所有权
            std::weak_ptr<Entity> weakEntity = entity;
            e1 = entity;
        } // 退出该部分时，entity 被销毁，引用计数减一
    } // 退出该部分时，e1 才被销毁，引用计数减至0，shared_ptr 分配的空间才被销毁
}
