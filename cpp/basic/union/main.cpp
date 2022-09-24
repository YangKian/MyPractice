#include <iostream>

struct Vector2 {
    float x, y;
};

// Vector4 有一个成员，是一个联合体
// 联合体是指，联合体中定义的不同变量，共享同一片内存
struct Vector4 {
    union {
        // 第一个变量，4 个值分配 32 byte 内存
        struct {
            float x, y, z, w;
        };
        // 第二个变量，2 个值分配 32 byte 内存
        struct {
            Vector2 a, b;
        };
    };
};

void PrintVector2(const Vector2& vector) {
    std::cout << vector.x << ", " << vector.y << std::endl;
}

int main() {
    Vector4 vector = { 1.0f, 2.0f, 3.0f, 4.0f };
    PrintVector2(vector.a); // -> 1, 2
    PrintVector2(vector.b); // -> 3, 4
    vector.z = 400.0f;
    std::cout << "-----------------" << std::endl;
    PrintVector2(vector.a); // -> 1, 2
    PrintVector2(vector.b); // -> 400, 4
}
