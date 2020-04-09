#include <iostream>
#include <string>

// 使用 void PrintString(std::string str) 的问题：
// 其实是传递了一个 str 的拷贝，会在堆上分配内存，速度很慢
// 改为使用常量引用，即避免了值拷贝，又满足了只读的要求
void PrintString(const std::string& str) {
    std::cout << str << std::endl;
}

int main() {
    const char* name = "Tom";
    std::cout << name << std::endl;
    //name[2] = 'a'; 错误的用法

    char name1[4] = {'T', 'o', 'm', '\0'};
    std::cout << name1 << std::endl;


    // 错误用法
//    char* name2 = "Tom";
//    name2[1] = 'a'; // 无法修改字符串常量，会报段错误
//    std::cout << name2 << std::endl;
    // 修正
    char name3[] = "Tom";
    name3[1] = 'a';
    std::cout << name3 << std::endl;

    std::string s_name = "Bob";
    // std::string s_wrong_use = "Bob" + "Tom"; 错误的用法，string 其实是 const char
    std::cout << s_name << std::endl;
    s_name += " Tom"; // 其实是创建了一个新的 s_name
    std::cout << s_name << std::endl;

    PrintString(s_name);
}
