#include <iostream>
#include <cstring>

class String {
private:
    char* m_Buffer;
    unsigned int m_Size;
public:
    String(const char* string) {
        m_Size = strlen(string);
        m_Buffer = new char[m_Size + 1];
        memcpy(m_Buffer, string, m_Size + 1);
    }

    // 使用拷贝构造器来解决结构体深拷贝的问题
    // 错误的写法，并没有实现深拷贝
//    String(const String& other) : m_Buffer(other.m_Buffer), m_Size(other.m_Size) {}
    // 正确的写法
    String(const String& other) : m_Size(other.m_Size) {
        m_Buffer = new char[m_Size + 1];
        memcpy(m_Buffer, other.m_Buffer, m_Size + 1);
        std::cout << "Copied String!" << std::endl;
    }

    // 如果要禁止对结构体进行深拷贝
    // String(const String& other) = delete;
    // 做了上述声明后，会禁用拷贝赋值：String second = str

    ~String() {
        delete[] m_Buffer;
    }

    // 重载[]运算符
    char& operator[](unsigned int index) {
        return m_Buffer[index];
    }

    // 使用 friend 关键字后，声明的方法中可以访问对应类的私有变量
    friend std::ostream& operator<<(std::ostream& stream, const String& string);
};

std::ostream& operator<<(std::ostream& stream, const String& string) {
    stream << string.m_Buffer;
    return stream;
}

/**
 * TIPS: 在传递对象作为参数时，绝大部分情况下请使用常量引用
 */
// 使用常量引用来传递参数，避免不必要的拷贝和内存分配
void PrintString(const String& str) {
    std::cout << str << std::endl;
}

int main() {
    String str = "Tom";

    // 错误的拷贝方法，因为 String 的成员中 m_Buffer 是指针，当拷贝 String 时，等于对指针进行了拷贝，即，现在有两个指针同时
    // 指向了 m_Buffer 这块内存空间，当程序结束时两个String变量都会调用析构函数，其中一个先完成了 m_Buffer 的空间释放，这时
    // 第二个变量又试图去释放 m_Buffer，就会导致去释放一块已经被释放的内存，出错
    // String second = str;

    // 加上拷贝构造器后
    String second = str;
    second[1] = 'a';

    PrintString(str);
    PrintString(second);
}
