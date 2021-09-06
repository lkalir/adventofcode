cmake_minimum_required(VERSION 3.0)
project(adventofcode C CXX)

enable_testing()

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wpedantic -Wextra -Werror -march=native")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wpedantic -Wextra -Werror -march=native")
set(CMAKE_EXPORT_COMPILE_COMMANDS on)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

include_directories(src/ src/aoc_2015 /usr/local/include)
file(GLOB_RECURSE ALL_SRC src/aoc*/*.c)
add_library(aoc SHARED ${ALL_SRC})
add_executable(runner src/main.c)
target_link_libraries(runner aoc aocdata)

add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

