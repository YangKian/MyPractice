#include <iostream>
#include <cstring>
#include <cassert>

class String {
public:
    String() = default;

    String(const char* string) {
        printf("Created!\n");
        m_Size = strlen(string);
        m_Data = new char[m_Size];
        memcpy(m_Data, string, m_Size);
    };

    // 拷贝构造
    String(const String& other) {
        printf("Copyed!\n");
        m_Size = other.m_Size;
        m_Data = new char[m_Size];
        memcpy(m_Data, other.m_Data, m_Size);
    };

    // 右值引用
    String(String&& other) noexcept {
        printf("Moved!\n");
        m_Size = other.m_Size;
        m_Data = other.m_Data;

        // 注意：因为使用了 Move 语义，other 的所有权发生了转移，
        // 需要我们自己管理
        other.m_Size = 0;
        // 析构函数删除一个 nullptr，不会造成其他影响
        other.m_Data = nullptr;
    };

    String& operator=(String&& other) noexcept {
        printf("Moved!!!\n");

        if (this != &other) {
            // 注意：在赋值时，当前 m_Data 中已经分配了内存，因此需要先释放掉这部分的内存
            delete[] m_Data;
            m_Size = other.m_Size;
            m_Data = other.m_Data;

            // 注意：因为使用了 Move 语义，other 的所有权发生了转移，
            // 需要我们自己管理
            other.m_Size = 0;
            // 析构函数删除一个 nullptr，不会造成其他影响
            other.m_Data = nullptr;
        }
        return *this;
    };

    ~String() {
        printf("Destroy!\n");
        delete[] m_Data;
    }

    void Print() {
        for (uint32_t i = 0; i < m_Size; i++)
            printf("%c", m_Data[i]);
        printf("\n");
    }

private:
    char * m_Data;
    uint32_t m_Size;
};

class Entity {
public:
    Entity(const String& name)
        : m_Name(name)
    { }

    Entity(String&& name)
            : m_Name(std::move(name))
    { }

    void PrintName() {
        m_Name.Print();
    }

private:
    String m_Name;
};

int main() {
    // 存在的问题：会分配两次内存
    {
        String name = String("Tom");
        Entity entity(name);
        entity.PrintName();
    }
    printf("==================\n");

    {
        Entity entityMoved("Tom");
        entityMoved.PrintName();
    }
    printf("==================\n");

    {
        String string = "Hello";
        // 等效写法：String dest = (String&&)string;
        String dest = std::move(string);
    }
    printf("==================\n");

    String apple = "Apple";
    String d;

    apple.Print();
    d.Print();
    d = std::move(apple);
    apple.Print();
    d.Print();

    return 0;
}
