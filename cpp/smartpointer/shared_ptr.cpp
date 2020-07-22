//#include <string>
//#include <vector>
//#include <memory>
//#include <iostream>
//
//using std::string;
//using std::vector;
//using std::make_shared;
//using std::shared_ptr;
//
//class StrBlob {
//public:
//    typedef vector<string>::size_type size_type;
//
//    StrBlob();
//    StrBlob(std::initializer_list<string> l1);
//
//    size_type size() const { return data->size(); };
//    bool empty() const { return data->empty(); };
//
//    void push_back(const string &t) { data->emplace_back(t); };
//    void pop_back();
//
//    string& front();
//    string& back();
//
//private:
//    shared_ptr<vector<string>> data;
//
//    //如果 data[i] 不合法，抛出一个异常
//    void check(size_type i, const string &msg) const;
//};

#include "shared_ptr.h"

StrBlob::StrBlob() : data(make_shared<vector<string>>()) {};
StrBlob::StrBlob(std::initializer_list<string> l1) : data(make_shared<vector<string>>(l1)) {}

void StrBlob::check(size_type i, const string &msg) const {
    if (i >= data->size())
        std::cout << "Err: " << msg << std::endl;
}

string & StrBlob::front() {
    check(0, "front on empty StrBlob");
    return data->front();
}

string & StrBlob::back() {
    check(0, "back on empty StrBlob");
    return data->back();
}

void StrBlob::pop_back() {
    check(0, "pop_back on empty StrBlob");
    data->pop_back();
}