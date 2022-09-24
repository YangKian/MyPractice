#include <iostream>
#include <vector>
#include <algorithm>

struct Vertex {
    float x, y, z;

    Vertex(float a, float b, float c) : x(a), y(b), z(c) {}

    Vertex(const Vertex& vertex) : x(vertex.x), y(vertex.y), z(vertex.z) {
        std::cout << "Copied now !" << std::endl;
    }
};

std::ostream& operator<<(std::ostream& stream, const Vertex& vertex) {
    stream << vertex.x << ", " << vertex.y << ", " << vertex.z;
    return stream;
}

// 使用引用传参避免不必要的拷贝
void Function(const std::vector<Vertex>& v) {}

void printCommaSeparated(const std::string& text) {
    std::cout << text << ", ";
}

int main() {
    std::vector<Vertex> vertices;
    vertices.reserve(3); // 优化一：在知晓vector大小的情况下，提前为 vector 分配空间，类似于go中的 make([]int, 0, 3)

    // 优化二：使用 .emplace_back() 替换 .push_back()
    // push_back()：在外部调用元素类型的构造器创建实例，然后将实例拷贝到 vector 中
    // vertices.push_back(Vertex(1, 2, 3)) 拷贝构造器会触发
    // emplace_back()：将参数传递给元素类型的构造器，然后在 vector 管理的内存空间中直接构造对象
    vertices.emplace_back(1, 2, 3);
    vertices.emplace_back(4, 5, 6);
    vertices.emplace_back(7, 8, 9);

    for (auto & vertice : vertices) {
        std::cout << vertice << std::endl;
    }

    // 删除指定的元素（使用迭代器和 offset ）
    vertices.erase(vertices.begin() + 1);

    // 使用引用避免拷贝
    for (const auto& vertice : vertices) {
        std::cout << vertice << std::endl;
    }

    vertices.clear();

    // 打印容器的内容
    const std::vector<std::string> names {"Peter", "Harry", "Julia", "Marc"};
    std::for_each(std::begin(names), std::end(names), printCommaSeparated);

    // 判断两个序列是否相等
    const std::vector<std::string> names1 {"Peter", "Tom", "Julia", "Marc"};
    const bool isEqual = std::equal(std::begin(names), std::end(names), std::begin(names1), std::end(names1));
    std::cout << isEqual << std::endl;

    const bool isEqual1 = std::equal(std::begin(names), std::end(names), std::begin(names1), std::end(names1),
                                    [](const std::string& s1, const std::string& s2) {
                                        return s1.compare(0, 3, s2, 0, 3) == 0;
                                    });
    std::cout << isEqual1 << std::endl;
}
