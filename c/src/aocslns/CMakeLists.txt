add_subdirectory(aoc_2015)
add_library(aoc-aocslns aocslns.c)
target_link_libraries(aoc-aocslns Aoc::Aoc2015)
add_library(Aoc::Aocslns ALIAS aoc-aocslns)
