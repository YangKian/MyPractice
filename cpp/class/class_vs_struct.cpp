#include <iostream>

class PlayerClass
{
public: //类中的成员默认是私有的
    int x, y;
    int speed;

    void move(int xa, int ya)
    {
        x += xa * speed;
        y += ya * speed;
    }
};

struct PlayerStruct
{
//结构体中的成员默认是共有的
    int x, y;
    int speed;

    void move(int xa, int ya)
    {
        x += xa * speed;
        y += ya * speed;
    }
};

int test() {
    PlayerClass player1;
    player1.move(2, 3);

    PlayerStruct player2;
    player2.move(2, 3);

    std::cin.get();
}
