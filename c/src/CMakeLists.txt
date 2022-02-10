add_compile_options(-Wall -pedantic -Wextra -Werror -march=native)

add_subdirectory(aocslns)

add_executable(runner main.c)
target_link_libraries(runner Aoc::Aocslns aocdata)
