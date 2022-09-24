cmake_minimum_required(VERSION 3.16) # 指定支持 CMake 的最小版本
project(hello_library) # 指定当前项目的名称，便于在有多个项目时可以方便的引用特定的变量

set(CMAKE_CXX_STANDARD 14)

############################################################
# Create a library
############################################################

# ========================= 创建静态链接库 ===========================

# add_library() 函数用来根据源文件创建一个库
add_library(hello_library STATIC
        src/Hello.cpp)


# ========================= 创建动态链接库 ===========================

add_library(hello_library_dyn SHARED
        src/Hello_shared.cpp)

# 可以为动态链接库添加别名
add_library(hello::dyn_library ALIAS hello_library_dyn)

# 使用 target_include_directories 来指定编译时需要用到的目录
# 这些目录会被用于：
#   - 编译库时
#   - 编译链接该库的任意其它目标时
# scopes 域的含义：
#   - PRIVATE： the directory is added to this target’s include directories
#   - INTERFACE：the directory is added to the include directories for any targets that link this library.
#   - PUBLIC： As above, it is included in this library and also any targets that link this library.
target_include_directories(hello_library
        PUBLIC
        ${PROJECT_SOURCE_DIR}/include
        )

############################################################
# Create an executable
############################################################

# 指定了一个可执行文件应该由哪些源文件来构建
# 第一个参数是创建的可执行文件的名字，后面的参数是需要编译的源文件列表
# 如果使用项目名做为可执行文件的名字，则第一个参数可以使用 ${PROJECT_NAME}
add_executable(hello_binary src/main.cpp)

# 使用 GLOB 指令和正则匹配来指定文件
# file(GLOB SOURCES src/*.cpp)

# 在链接时需要链接 hello_library 库
target_link_libraries( hello_binary
        PRIVATE
        hello_library)

# 在链接时加入动态链接库
target_link_libraries( hello_binary
        PRIVATE
        hello::dyn_library)
