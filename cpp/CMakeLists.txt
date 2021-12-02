cmake_minimum_required(VERSION 3.0)
project(adventofcode CXX)

find_package(OpenSSL REQUIRED)
find_package(Boost REQUIRED)

enable_testing()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wpedantic -Wextra -Werror -march=native")
set(CMAKE_EXPORT_COMPILE_COMMANDS on)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

include_directories(src/ src/aoc_2015 src/aoc_2019 src/util /usr/local/include)
file(GLOB_RECURSE ALL_SRC src/aoc*/*.cpp src/util/*.cpp)
add_library(aoc STATIC ${ALL_SRC})

add_executable(runner src/main.cpp)
target_link_libraries(runner aoc aocdata OpenSSL::SSL Boost::boost)

add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

