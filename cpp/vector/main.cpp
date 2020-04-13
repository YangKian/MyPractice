#include <iostream>
#include <vector>

struct Vertex {
    float x, y, z;
};

std::ostream& operator<<(std::ostream& stream, const Vertex& vertex) {
    stream << vertex.x << ", " << vertex.y << ", " << vertex.z;
    return stream;
}

// 使用引用传参避免不必要的拷贝
void Function(const std::vector<Vertex>& v) {}

int main() {
    std::vector<Vertex> vertices;
    vertices.push_back({1, 2, 3});
    vertices.push_back({2, 4, 7});

    for (int i = 0; i < vertices.size(); i++) {
        std::cout << vertices[i] << std::endl;
    }

    // 删除指定的元素（使用迭代器和 offset ）
    vertices.erase(vertices.begin() + 1);

    // 使用引用避免拷贝
    for (const auto& vertice : vertices) {
        std::cout << vertice << std::endl;
    }

    vertices.clear();
}
