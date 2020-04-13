#include <iostream>

/**
 * 运算符的本质其实是函数，如：+ 对应的是 add()
 */

struct Vector {
    float x, y;

    Vector(float a, float b) : x(a), y(b) {}

    Vector Add(const Vector& other) const {
        return Vector(other.x + x, other.y + y);
    }

    Vector operator+(const Vector& other) const {
        return Add(other);
    }

    Vector Multiply(const Vector& other) const {
        return Vector(other.x * x, other.y * y);
    }

    Vector operator*(const Vector& other) const {
        return Multiply(other);
    }

    bool operator==(const Vector& other) const {
        return x == other.x && y == other.y;
    }

    bool operator!=(const Vector& other) const {
        return !(*this == other); // this 关键字是一个指向当前对象实例的指针
    }
};

std::ostream& operator<<(std::ostream& stream, const Vector& vector) {
    stream << vector.x << ", " << vector.y;
    return stream;
}

int main() {

    Vector position(4.0f, 5.0f);
    Vector speed(0.5f, 1.5f);
    Vector powerUp(1.1f, 1.1f);

    Vector result = position.Add(speed.Multiply(powerUp));
    Vector result1 = position + speed * powerUp;

    std::cout << result << std::endl;
    return 0;
}
