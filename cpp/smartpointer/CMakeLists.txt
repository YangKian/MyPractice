cmake_minimum_required(VERSION 3.15)
project(smartpointer)

set(CMAKE_CXX_STANDARD 14)

add_executable(smartpointer main.cpp shared_ptr.cpp weak_ptr.cpp shared_ptr.h weak_ptr.h)