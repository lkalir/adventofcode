file(GLOB SRCS *.c)
add_library(aoc-2015 ${SRCS})
add_library(Aoc::Aoc2015 ALIAS aoc-2015)
