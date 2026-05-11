#include "splines/spline.h"
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>
using Catch::Matchers::WithinAbs;


TEST_CASE("Basic spline creation") {
    Eigen::Vector<double, 4> x{1.0, 2.0, 3.0, 4.0};
    Eigen::Vector<double, 4> y{1.0, 8.0, 27.0, 64.0};

    uint8_t order = 3;
    Eigen::Vector<double, 1> derivative_equality_at_start{2.0};
    Eigen::Vector<double, 1> derivative_value_at_start{0.0};
    Eigen::Vector<double, 1> derivative_equality_at_end{2.0};
    Eigen::Vector<double, 1> derivative_value_at_end{0.0};
    Spline spline(x, y, order, derivative_equality_at_start, derivative_value_at_start, derivative_equality_at_end, derivative_value_at_end);
    REQUIRE_THAT(spline.evaluate(1.5), WithinAbs(1.5*1.5*1.5, 0.1));
    spline.evaluate_derivative(1.5, static_cast<uint8_t>(1));
}

TEST_CASE("evaluation") {
    Eigen::Vector<double, 4> x{1.0, 2.0, 3.0, 4.0};
    Eigen::Vector<double, 4> y{1.0, 8.0, 27.0, 64.0};

    uint8_t order = 3;
    Spline spline(x, y, order);
    REQUIRE_THAT(spline.evaluate(1.5), WithinAbs(1.5*1.5*1.5, 0.1));
    spline.evaluate_derivative(1.5, static_cast<uint8_t>(1));
}