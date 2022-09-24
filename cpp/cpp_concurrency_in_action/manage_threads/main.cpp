#include <iostream>
#include <thread>

void do_something() {
    std::cout << "do something" << std::endl;
}

class BackgroundTask
{
public:
    void operator() () const {
        do_something();
    }
};

void launch_thread() {
    // std::thread 可以接收一个 callable type 做参数，
    BackgroundTask t;
    std::thread thread1(t);
    thread1.join();

    // 注意以下写法是错误的：传递一个临时对象会触发 "C++‘s most vexing parse“，
    // 编译器会将其解释为函数声明而不是对象定义
    // std::thread thread(BackgroundTask());
    // 正确的用法：
    std::thread thread2{BackgroundTask()};
    thread2.join();

    // 使用 lambda 表达式构造线程
    std::thread thread3([] {
        do_something();
    });
    thread3.join();
}

void do_something_with_args(int i, const std::string& s) {}

void launch_thread_with_argument() {
    char buffer[1024];
    sprintf(buffer, "%i", 100);
    // 1. 默认情况下，参数被 **拷贝** 到线程对象的内部存储中，线程执行时会访问它们，然后
    // 把它们当成 **右值** 传递给可执行对象或者函数。即使可执行对象/函数需要一个引用参数，
    // 也是如此
    // 2. 注意，如果直接将 buffer 作为参数传给线程构造器，实际上传递的是数组对象的指针，
    // 可能导致 launch_thread_with_argument 函数退出后，do_something_with_args 函数
    // 还未开始执行，此时再次访问局部变量 buffer 的行为是未定义的。因此需要先将 buffer 
    // 转换为 string
    // 3. 如果 do_something_with_args 的引用参数不是常量引用，则编译会失败。因为线程构造器
    // 内部将拷贝传入的参数当成右值传给 do_something_with_args，而非常量引用不能接受右值。
    // 如果要传递非常量引用，使用 std::ref()
    // 4. std::thread 构造函数的机制与 std::bind 相同
    std::thread thread(do_something_with_args, 100, std::string(buffer));
    thread.detach();
}

class X
{
public:
    void do_work() {};
};

void launch_thread_with_class_method() {
    X x;
    std::thread t(&X::do_work, &x);
    t.join();
}

void get_thread_id() {
    std::cout << "current thread id: "
              << std::this_thread::get_id()
              << std::endl;

    std::thread thread([] {
        do_something();
    });
    std::cout << "thread id: "
              << thread.get_id()
              << std::endl;
    thread.join();
}

int main() {
    launch_thread();
    launch_thread_with_argument();
    launch_thread_with_class_method();
    get_thread_id();
    return 0;
}

// std::thread 的实例是 movable，但不是 copyable
