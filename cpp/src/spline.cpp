#include "splines/spline.h"
Spline::Spline(const Eigen::VectorXd &x, const Eigen::VectorXd &y, uint8_t order) {
    // Implementation of the constructor
    handle_ = spline_new(order, x.data(), y.data(), x.size(), nullptr, nullptr, nullptr, nullptr, 0, 0);
}

Spline::Spline(const Eigen::VectorXd &x, const Eigen::VectorXd &y, uint8_t order, 
               const Eigen::VectorXd &derivative_equality_at_start, const Eigen::VectorXd &derivative_value_at_start, 
               const Eigen::VectorXd &derivative_equality_at_end, const Eigen::VectorXd &derivative_value_at_end) {
    // Implementation of the constructor with derivative constraints
    handle_ = spline_new(order, x.data(), y.data(), x.size(), 
                          derivative_equality_at_start.data(), derivative_value_at_start.data(), 
                          derivative_equality_at_end.data(), derivative_value_at_end.data(), 
                          derivative_equality_at_start.size(), derivative_equality_at_end.size());
}


Spline::~Spline() {
    // Implementation of the destructor
    spline_free(handle_);
}

double Spline::evaluate(double x) const {
    // Implementation of the evaluate method
    return spline_evaluate(handle_, x);
}

double Spline::evaluate_derivative(double x, uint8_t order) const {
    // Implementation of the evaluate_derivative method
    return spline_evaluate_derivative(handle_, x, order);
}