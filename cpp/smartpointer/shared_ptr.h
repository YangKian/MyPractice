#include <string>
#include <vector>
#include <memory>
#include <iostream>

using std::string;
using std::vector;
using std::make_shared;
using std::shared_ptr;

#ifndef SMARTPOINTER_SHARED_PTR_H
#define SMARTPOINTER_SHARED_PTR_H

class StrBlob {
public:
    typedef vector<string>::size_type size_type;

    StrBlob();
    StrBlob(std::initializer_list<string> l1);

    size_type size() const { return data->size(); };
    bool empty() const { return data->empty(); };

    void push_back(const string &t) { data->emplace_back(t); };
    void pop_back();

    string& front();
    string& back();

    friend class StrBlobPtr;
private:
    shared_ptr<vector<string>> data;

    //如果 data[i] 不合法，抛出一个异常
    void check(size_type i, const string &msg) const;
};


#endif //SMARTPOINTER_SHARED_PTR_H
