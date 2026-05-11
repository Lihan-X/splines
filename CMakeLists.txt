cmake_minimum_required(VERSION 3.16)
project(SplinesCpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Rust library output paths
set(RUST_PROJECT_DIR ${CMAKE_SOURCE_DIR}/rust)


set(RUST_LIB ${RUST_PROJECT_DIR}/target/release/libspline_rs.so)

# Build Rust with cargo
add_custom_command(
    OUTPUT ${RUST_LIB}
    COMMAND cargo build --release
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    COMMENT "Building Rust library"
)

add_custom_target(spline_rs ALL DEPENDS ${RUST_LIB})

# C++ library
add_library(${PROJECT_NAME} SHARED
    cpp/src/spline.cpp
)

add_dependencies(${PROJECT_NAME} spline_rs)

target_include_directories(${PROJECT_NAME} PUBLIC
    ${CMAKE_SOURCE_DIR}/cpp/include
)

target_link_directories(${PROJECT_NAME} PRIVATE
    ${RUST_PROJECT_DIR}/target/release
)

target_link_libraries(${PROJECT_NAME} PRIVATE ${RUST_LIB})

install(
        TARGETS
        ${PROJECT_NAME}
        RUNTIME DESTINATION bin
        ARCHIVE DESTINATION lib
        LIBRARY DESTINATION lib
)

install(
        DIRECTORY cpp/include/
        DESTINATION include
)

install(
    FILES ${RUST_LIB}
    DESTINATION lib
)

# tests
include(FetchContent)

FetchContent_Declare(
    Catch2
    GIT_REPOSITORY https://github.com/catchorg/Catch2.git
    GIT_TAG v3.5.4
)

FetchContent_MakeAvailable(Catch2)
add_executable(test_splines
    tests/cubic_spline.cpp
)
target_link_libraries(test_splines PRIVATE
    ${PROJECT_NAME}
    Catch2::Catch2WithMain
)
add_test(NAME test_splines COMMAND test_splines)
enable_testing()