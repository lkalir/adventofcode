file(GLOB SRCS *.cpp)
add_library(aoc_2019 ${SRCS})
add_library(AOC::aoc2019 ALIAS aoc_2019)
