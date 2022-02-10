add_compile_options(-Wall -Wpedantic -Wextra -Werror -march=native)

include_directories(util)

add_subdirectory(util)
add_subdirectory(aoc_2015)
add_subdirectory(aoc_2019)

add_executable(runner main.cpp)
target_link_libraries(runner AOC::utils AOC::aoc2015 AOC::aoc2019 aocdata OpenSSL::SSL Boost::boost)

