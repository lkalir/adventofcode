file(GLOB SRCS *.cpp)
add_library(aoc_2015 ${SRCS})
add_library(AOC::aoc2015 ALIAS aoc_2015)
target_link_libraries(aoc_2015 AOC::utils)
