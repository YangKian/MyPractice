cmake_minimum_required(VERSION 3.16)
project(manage_threads)

find_package(Threads REQUIRED)

# use for debug
set (CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fsanitize=address")

set (CMAKE_CXX_STANDARD 17)

add_executable(manage_threads main.cpp)
target_link_libraries(Threads::Threads)
