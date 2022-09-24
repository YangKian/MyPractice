#include <iostream>\

class Entity {
public:
    float X, Y;

    Entity()
    {
        X = 0.0f, Y = 0.0f;
    }

    Entity(float x, float y)
    {
        X = x, Y = y;
    }

    void print()
    {
        std::cout << X << ", " << Y << std::endl;
    }

    ~Entity()
    {
        std::cout << "destroy" << std::endl;
    }
};

int main() {
    Entity e;
    e.print();

    Entity e1(10.1f, 20.5f);
    e1.print();
}
