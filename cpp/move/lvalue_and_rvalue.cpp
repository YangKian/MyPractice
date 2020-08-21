#include <iostream>

int GetValue() {
    return 10;
}

int& RightFunction() {
    int* a = new int;
    *a = 10;
    return reinterpret_cast<int &>(a);
}

void SetValue(int& value) {}

// 参数既可以是左值，也可以是右值
void SetValueWithRValueOrLValue(const int& value) {}

// 参数只能是左值
void PrintName(std::string& name) {}

// 参数只能是右值，这种用法称为右值引用
void PrintNameWithRvalueReferrence(std::string&& name) {
    std::cout << "I'm rvalue" << std::endl;
}
// 只要存在右值引用的函数，则右值引用永远只会走上面那个函数
void PrintNameWithRvalueReferrence(const std::string& name) {
    std::cout << "I'm lvalue" << std::endl;
}

int test() {
    // 理解左值和右值：
    // 简单理解：能用在等号左边的是左值，不能用在等号左边的是右值
    // 左值是地址，可以存储左值或者右值，右值是临时变量，不能对其取值引用
    int i = GetValue();
    // GetValue() = 10(); 错误，不能将右值赋值给右值
    RightFunction() = 10;

    int a = 10;
    // int& b = 10; 错误
    // 特殊用法，实际上编译器做的是：
    // int tmp = 10;
    // int& b = &tmp;
    const int& b = 10;
    SetValue(a);
//    SetValue(10); 错误
    SetValueWithRValueOrLValue(10);

    std::string firstName = "Tom";
    std::string lastName = "Jerry";
    std::string fullName = firstName + lastName;
    PrintName(fullName);
    // PrintName(firstName + lastName); 错误，firstName + lastName 的结果是右值

    PrintNameWithRvalueReferrence(firstName + lastName);
    // PrintNameWithRvalueReferrence(fullName); 错误，只能传入右值
}