cmake_minimum_required(VERSION 3.17)
project(move)

set(CMAKE_CXX_STANDARD 14)

add_executable(move main.cpp lvalue_and_rvalue.cpp)