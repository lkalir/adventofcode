cmake_minimum_required(VERSION 3.0)
project(adventofcode C CXX)

enable_testing()

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS on)

include_directories(src /usr/local/include)

add_subdirectory(src)

add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

