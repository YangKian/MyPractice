#include <iostream>

class Entity {
private:
    int m_X, m_Y;
    int* p1, *p2; // 注意声明两个指针变量时，两个变量前都要加 *，否则没加的那个不是指针变量
    mutable int var; // 使用 mutable 关键字声明该变量是可以被修改的
public:
    // const_and_mutable 用在类的方法中，放在方法名之后，声明该方法是只读的，不能通过该方法去修改类的成员变量
    int GetX() const {
        // m_X = 2; 错误用法
        return m_X;
    }

    int GetY() {
        return m_Y;
    }

    // 对于只读函数中还需要修改类变量的情况，在需要修改的类变量前加上 mutable 关键字-p-p
    int Get() const {
        var = 2;
        return m_X;
    }
};

// 在形参前加 const_and_mutable，说明该形参是不可变的
// const_and_mutable Entity& e 等价于 const_and_mutable Entity * e，即不能修改值，但是可以修改指向
void PrintClass(const Entity& e) {
    std::cout << e.GetX() << std::endl;
}

// 错误的用法，形参中声明不可变，但是类方法 GetY 没有声明只读，即无法保证不可变，可能在类方法中就做了其他改动
// 因此会报错
void PrintClassWrong(const Entity& e) {
    std::cout << e.GetY() << std::endl;
}


int main() {

    const int MAX_AGE = 90;

    int* a = new int;
    // a = &MAX_AGE 是错误的，因为 int* 与 const_and_mutable int* 类型不匹配
    a = (int*)&MAX_AGE; // 尽管声明了变量是 const_and_mutable 的，但是依旧可以打破该声明，const_and_mutable 只是约定

    // const_and_mutable 放在*前，说明不能修改该指针变量的内容，但是可以修改该指针变量的指向
    const int* b = new int; // 等价的写法：int const_and_mutable* b = new int;
    b = a;
    *b = 2;

    // const_and_mutable 放在*后，说明不能修改该指针变量的指向，但是可以修改其内容；
    int* const c = new int;
    c = a;
    *c = 2;

    const int* const d = new int; // 既不能修改指向，也不能修改内容

    std::cout << *a << std::endl;
    return 0;
}
