file(GLOB_RECURSE ALL_BENCH_CPP bench*.cpp)

foreach(ONE_BENCH_CPP ${ALL_BENCH_CPP})
  get_filename_component(ONE_BENCH_EXEC ${ONE_BENCH_CPP} NAME_WE)

  # Avoid name collision
  set(TARGET_NAME Bench_${ONE_BENCH_EXEC})
  add_compile_options("-Wall" "-Wpedantic" "-Wextra" "-std=c++20" "$<$<CONFIG:RELEASE>:-march=native;>")

  add_executable(${TARGET_NAME} ${ONE_BENCH_CPP})
  set_target_properties(${TARGET_NAME} PROPERTIES OUTPUT_NAME ${ONE_BENCH_EXEC})
  target_link_libraries(
    ${TARGET_NAME}
    benchmark
    aoc
    aocdata
    ${CMAKE_THREAD_LIBS_INIT}
  )
endforeach()

# Download and unpack googletest at configure time
configure_file(CMakeLists.txt.in googletest-download/CMakeLists.txt)
execute_process(
  COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" .
  RESULT_VARIABLE result
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/googletest-download
)
if(result)
  message(FATAL_ERROR "CMake step for googletest failed: ${result}")
endif()
execute_process(
  COMMAND ${CMAKE_COMMAND} --build .
  RESULT_VARIABLE result
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/googletest-download
)
if(result)
  message(FATAL_ERROR "Build step for googletest failed: ${result}")
endif()

# Prevent overriding the parent project's compiler/linker
# settings on Windows
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

# Add googletest directly to our build. This defines
# the gtest and gtest_main targets.
add_subdirectory(
  ${CMAKE_CURRENT_BINARY_DIR}/googletest-src
  ${CMAKE_CURRENT_BINARY_DIR}/googletest-build
  EXCLUDE_FROM_ALL
)

# The gtest/gtest_main targets carry header search path
# dependencies automatically when using CMake 2.8.11 or
# later. Otherwise we have to add them here ourselves.
if(CMAKE_VERSION VERSION_LESS 2.8.11)
  include_directories("${gtest_SOURCE_DIR}/include")
endif()

file(GLOB_RECURSE ALL_TEST_CPP test*.cpp)

foreach(ONE_TEST_CPP ${ALL_TEST_CPP})
  get_filename_component(ONE_TEST_EXEC ${ONE_TEST_CPP} NAME_WE)

  # Avoid name collision
  set(TARGET_NAME Test_${ONE_TEST_EXEC})
  add_compile_options("-Wall" "-Wpedantic" "-Wextra" "-std=c++20" "$<$<CONFIG:RELEASE>:-march=native;>")
  add_executable(${TARGET_NAME} ${ONE_TEST_CPP})
  set_target_properties(${TARGET_NAME} PROPERTIES OUTPUT_NAME ${ONE_TEST_EXEC})
  target_link_libraries(
    ${TARGET_NAME}
    gtest
    aoc
    aocdata
    ${CMAKE_THREAD_LIBS_INIT}
  )

  add_test(NAME ${TARGET_NAME} COMMAND ${ONE_TEST_EXEC})
endforeach()
