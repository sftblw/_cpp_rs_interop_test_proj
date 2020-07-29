cmake_minimum_required(VERSION 3.16)
project(CppRsInteropTest)

set(CMAKE_CXX_STANDARD 14)

add_library(MyLibProjCMakeTargetName library.cpp library.h)

install(TARGETS MyLibProjCMakeTargetName DESTINATION .)