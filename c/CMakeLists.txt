cmake_minimum_required(VERSION 3.0)
project(adventofcode C CXX)

enable_testing()

add_compile_options("-Wall" "-Wpedantic" "-Wextra" "-Werror" "$<$<CONFIG:RELEASE>:-march=native;>")
set(CMAKE_EXPORT_COMPILE_COMMANDS on)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

include_directories(src/ src/aoc_2015 /usr/local/include)
file(GLOB_RECURSE ALL_SRC src/aoc*/*.c)
add_library(aoc SHARED ${ALL_SRC})
add_executable(runner src/main.c)
target_link_libraries(runner aoc aocdata)

add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

