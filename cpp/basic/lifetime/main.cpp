#include <iostream>

int* create() {
    int array[50];
    return array;
}

void create_with_modify(int* array) {
    // fill array
}

int* create_in_heap() {
    int* array = new int[50];
    return array;
}

int main() {
    // 尽管 create 返回的是指针，但是这是一个指向栈空间的指针
    // 随着create()函数退出，栈空间会被销毁，该指针也会失效
    int* a = create();

    //修正一：
    int array[50];
    create_with_modify(array);

    //修正二：
    int* array_heap = create_in_heap();
}
