#include "splines/spline.h"
Spline::Spline(const Eigen::VectorXd &x, const Eigen::VectorXd &y, uint8_t order) {
    // Implementation of the constructor
    handle_ = spline_new(order, x.data(), y.data(), x.size());
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