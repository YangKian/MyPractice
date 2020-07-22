#include <string>
#include <vector>
#include <memory>
#include <iostream>
#include "shared_ptr.h"

using std::string;
using std::vector;
using std::make_shared;
using std::shared_ptr;
using std::weak_ptr;


#ifndef SMARTPOINTER_WEAK_PTR_H
#define SMARTPOINTER_WEAK_PTR_H

class StrBlobPtr {
public:
    // 默认构造函数显示将 curr 初始化为 0，将 wptr 隐式初始化为一个空指针
    StrBlobPtr() : curr(0) {};
    // 使用共享指针来初始化弱指针
    StrBlobPtr(StrBlob &a, size_t sz = 0) :wptr(a.data), curr(sz) {}

    string& deref() const;
    StrBlobPtr& incr();
private:
    // 执行检查，如果检查成功，返回一个指向 vector 的 shared_ptr
    shared_ptr<vector<string>> check(std::size_t, const std::string&) const;
    // 保存一个弱指针，注意弱指针引用的对象可能会被销毁
    weak_ptr<vector<string>> wptr;
    // 返回在数组中的当前位置
    std::size_t curr;
};

#endif //SMARTPOINTER_WEAK_PTR_H
