#include <iostream>

/**
 * 可见性：
 *  - private：子类不可见，类外不可访问
 *  - protected：子类可见，类外不可访问
 *  - public：全部可见
 */

class Printable {
public:
    virtual std::string GetClassName() = 0; //纯虚函数，等同于interface，要求子类实现该接口
};

class Entity {
public:
    float X, Y;

    void Move(float x, float y)
    {
        X += x;
        Y += y;
    }

    // 在父类中使用虚函数定义的方法，可以在子类中被重写
    // 使用虚函数会引入额外开销，在极端情况下可能会造成性能影响
    virtual std::string GetName() { return "Entity"; }
};

class Player : public Entity, public Printable { // 注意继承 Printable 时也需要声明 public，否则会被定义为 private
public:
    const char* Name;

    void PrintName()
    {
        std::cout << Name << std::endl;
    }

private:
    std::string m_Name;
public:
    Player() { Name = "abc"; }

    Player(const std::string& name) : m_Name(name) {}

    std::string GetName() override { return m_Name; } // c++ 11 之后可以加上 override 关键字

    std::string GetClassName() override { return "Player"; }
};

void PrintName(Entity *e) {
    std::cout << e -> GetName() << std::endl;
}

void PrintClassName(Printable *p) { // 注意这里的参数只能用指针类型
    std::cout << p -> GetClassName() << std::endl;
}

int main() {
    Player player;
    player.X = 2;
    player.Y = 3;
    player.Move(5, 9);

    auto *p1 = new Player("Tom");
    PrintName(p1);

    auto *e = new Entity();
    PrintName(e);

    PrintClassName(p1);
}
