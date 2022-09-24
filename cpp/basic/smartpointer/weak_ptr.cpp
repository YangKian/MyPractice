#include "weak_ptr.h"

shared_ptr<vector<string>> StrBlobPtr::check(std::size_t i, const string &msg) const {
    // 不能使用弱指针直接访问对象，因为底层对象可能已经被删除，要先调用 .lock() 方法进行检查
    auto ret = wptr.lock();
    if (!ret) {
        throw std::runtime_error("unbound StrBlobPtr");
    }
    if (i >= ret->size()) {
        std::cout << "Error: " << msg << std::endl;
    }
    return ret;
}

string &StrBlobPtr::deref() const {
    auto p = check(curr, "dereference past end.");
    return (*p)[curr];
}

// 前缀递增，返回递增后的对象引用
StrBlobPtr &StrBlobPtr::incr() {
    check(curr, "increment past end of StrBlobPtr");
    ++curr;
    return *this;
}
