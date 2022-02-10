cmake_minimum_required(VERSION 3.0)
project(adventofcode CXX)

find_package(OpenSSL REQUIRED)
find_package(Boost REQUIRED)

enable_testing()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS on)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

include_directories(src /usr/local/include)

add_subdirectory(src)
add_subdirectory(tests)

