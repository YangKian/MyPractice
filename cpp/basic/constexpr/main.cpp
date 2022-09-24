#include <iostream>
#include <cmath>

// variable templates
template <typename T>
constexpr T pi = T(3.1415926535897932384626433L);

template <typename T>
constexpr T computeCircumference(const T radius) {
    return 2 * radius * pi<T>;
}

// constexpr function will be called at compile time
constexpr unsigned long long factorial(const unsigned short n) {
    return n > 1 ? n * factorial(n - 1) : 1;
}

// constexpr can be used as constructors and member functions.
// constexpr class can be used at compile time and runtime.
// Note: constexpr class is not allowed to define virtual member functions(
//   there is no polymorphism at compile time), and a constexpr class must
//   not have an explicitly defined destructor
class Rectangle {
public:
    constexpr Rectangle() = delete;
    constexpr Rectangle(const double width, const double height):
        width{width}, height{height} {}
    [[nodiscard]] constexpr double getWidth() const {return width;}
    [[nodiscard]] constexpr double getHeight() const {return height;}
    [[nodiscard]] constexpr double getArea() const {return width * height;}
    [[nodiscard]] constexpr double getLengthOfDiagonal() const {
        return std::sqrt(std::pow(width, 2.0) + std::pow(height, 2.0));
    }
private:
    double width;
    double height;
};

int main() {
    // constexpr function can also be used like ordinary functions with
    // non-const arguments at runtime
    unsigned short number = 6;
    auto result = factorial(number);
    constexpr auto result1 = factorial(10);
    std::cout << "result: " << result
              << ", result1: " << result1
              << std::endl;

//    const long double radius{10.0L};
//    constexpr long double circumference = computeCircumference(radius);
//    std::cout << circumference << std::endl;


    constexpr Rectangle footballPlayGround{48.76, 110.0};
    constexpr double area = footballPlayGround.getArea();
    constexpr double diagonal = footballPlayGround.getLengthOfDiagonal();
    std::cout << "area: " << area
              << ", diagonal: " << diagonal
              << std::endl;
    return 0;
}
