cmake_minimum_required(VERSION 3.10)
project(wtf VERSION 0.0.1)
add_executable(wtf src/main.cpp)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++17")
