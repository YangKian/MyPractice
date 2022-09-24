#include <iostream>
#include <chrono>

struct Timer {
    std::chrono::time_point<std::chrono::system_clock> end;
    std::chrono::time_point<std::chrono::system_clock> start;
    std::chrono::duration<float> duration;

    Timer()
    {
        start = std::chrono::high_resolution_clock::now();
    }

    ~Timer()
    {
        end = std::chrono::high_resolution_clock::now();
        duration = end - start;

        float ms = duration.count() * 1000.0f;
        std::cout << "Timer took" << ms << "ms" << std::endl;
    }
};

void Function() {
    Timer timer;

    for (int i = 0; i < 100; i++) {
        std::cout << "hello\n";
    }
}

int main() {
    Function();
}
