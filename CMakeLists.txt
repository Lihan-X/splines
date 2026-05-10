cmake_minimum_required(VERSION 3.16)
project(SplinesCpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Rust library output paths
set(RUST_PROJECT_DIR ${CMAKE_SOURCE_DIR})


set(RUST_LIB ${RUST_PROJECT_DIR}/target/release/liblibspline.so)

# Build Rust with cargo
add_custom_command(
    OUTPUT ${RUST_LIB}
    COMMAND cargo build --release
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    COMMENT "Building Rust library"
)

add_custom_target(libspline ALL DEPENDS ${RUST_LIB})

# C++ library
add_library(${PROJECT_NAME} SHARED
    cpp/src/spline.cpp
)

add_dependencies(${PROJECT_NAME} libspline)

target_include_directories(${PROJECT_NAME} PUBLIC
    ${CMAKE_SOURCE_DIR}/cpp
)

target_link_directories(${PROJECT_NAME} PRIVATE
    ${RUST_PROJECT_DIR}/target/release
)

target_link_libraries(${PROJECT_NAME} PRIVATE liblibspline.so)