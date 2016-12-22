cmake_minimum_required(VERSION 3.6)
project(geometry)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/base.cpp)

include_directories(.)
include_directories(src)
include_directories(SYSTEM "lib/libigl/include")
include_directories(SYSTEM "lib/eigen")

add_library(geometry ${SOURCE_FILES})
