cmake_minimum_required(VERSION 3.0)
project(adventofcode C CXX)

enable_testing()

option(MAKE_SHARED "make shared")

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS on)

add_compile_options(-Wall -pedantic -Wextra -Werror -march=native)

include_directories(src /usr/local/include)

add_subdirectory(src/aocslns)

add_executable(runner src/main.c)
target_link_libraries(runner Aoc::Aocslns aocdata)

add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

