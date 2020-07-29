#include <iostream>
#include <array>
#include <algorithm>

int main() {
    int example[5]; // 数组的本质其实是指针
    int* ptr = example;

    //如果使用 new 来分配空间，创建一个新数组，该数组实际上被创建在堆上，要记得回收内存
    int* another = new int[5];
    delete[] another;

    // c++ 11 之后可以使用标准库
    std::array<int, 5> std_array_without_initial; // 未初始化
    for (int & i : std_array_without_initial)
        i = 2;

    std::array<int, 5> std_array{}; // 已初始化

    int array[5] = {1, 2, 2, 2, 5};
    int *res = std::find(std::begin(array), std::end(array), 2);
    if (res != std::end(array)) {
        std::cout << "find: " << std::distance(array, res) << std::endl;
    }
}
