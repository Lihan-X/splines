cmake_minimum_required(VERSION 3.16)

project(splines
  VERSION 0.0.1
  LANGUAGES CXX
)

find_package(ament_cmake REQUIRED)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()


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
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/cpp/include>
    $<INSTALL_INTERFACE:include>
)

target_link_directories(${PROJECT_NAME} PRIVATE
    ${RUST_PROJECT_DIR}/target/release
)

target_link_libraries(${PROJECT_NAME} PRIVATE ${RUST_LIB})


## Installation
include(GNUInstallDirs)
set(INSTALL_CMAKE_CONFIG_DIR ${CMAKE_INSTALL_LIBDIR}/cmake/Splines)

install(
        TARGETS ${PROJECT_NAME} 
        EXPORT SplinesTargets
        RUNTIME DESTINATION ${CMAKE_INSTALL_LIBDIR}
        ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

install(
        DIRECTORY cpp/include/
        DESTINATION include
)

install(
    FILES ${RUST_LIB}
    DESTINATION lib
)

# Export for build tree
export(EXPORT SplinesTargets
  NAMESPACE Splines::
  FILE ${CMAKE_CURRENT_BINARY_DIR}/SplinesTargets.cmake
)

# Generate and install config files
include(CMakePackageConfigHelpers)
write_basic_package_version_file(${CMAKE_CURRENT_BINARY_DIR}/SplinesConfigVersion.cmake
  VERSION ${PROJECT_VERSION}
  COMPATIBILITY SameMajorVersion
)

configure_package_config_file(cmake/SplinesConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/SplinesConfig.cmake
  INSTALL_DESTINATION ${INSTALL_CMAKE_CONFIG_DIR}
)

install(FILES
  ${CMAKE_CURRENT_BINARY_DIR}/SplinesConfig.cmake
  ${CMAKE_CURRENT_BINARY_DIR}/SplinesConfigVersion.cmake
  DESTINATION ${INSTALL_CMAKE_CONFIG_DIR}
)

install(EXPORT SplinesTargets
  NAMESPACE Splines::
  DESTINATION ${INSTALL_CMAKE_CONFIG_DIR}
)

# Export targets for find_package
ament_export_targets(SplinesTargets HAS_LIBRARY_TARGET)
ament_export_libraries(${PROJECT_NAME})

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

ament_package()