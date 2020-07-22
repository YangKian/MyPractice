#include <iostream>

struct Vector3 {
    float x, y, z;

    Vector3()
        : x(10), y(11), z(12) {}
};

int main() {
    // stack
    int value = 5;
    int array[5];
    array[0] = 0;
    array[1] = 1;
    array[2] = 2;
    array[3] = 3;
    array[4] = 4;
    Vector3 vector{};


    // heap
    int *hvalue = new int;
    *hvalue = 5;
    int* harray = new int[5];
    harray[0] = 0;
    harray[1] = 1;
    harray[2] = 2;
    harray[3] = 3;
    harray[4] = 4;
    auto *hvector = new Vector3;

    delete hvalue;
    delete[] harray;
    delete hvector;
}
