PROJECT(double-conversion-wrapper)

cmake_minimum_required(VERSION 3.17.5)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_SOURCE_DIR .)

set(
  headers
  wrapper.hpp
)
add_subdirectory(double-conversion)
add_library(double-conversion-wrapper wrapper.cpp ${headers})
target_link_libraries(double-conversion-wrapper double-conversion)
install (TARGETS double-conversion-wrapper DESTINATION .)
