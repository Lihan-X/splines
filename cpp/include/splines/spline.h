#pragma once
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct SplineHandle;

extern "C" {

    SplineHandle *spline_new(uint8_t order, 
                             const double *x_ptr, 
                             const double *y_ptr, 
                             uintptr_t len, 
                             const double *derivative_equality_at_start_ptr, 
                             const double *derivative_value_at_start_ptr, 
                             const double *derivative_equality_at_end_ptr, 
                             const double *derivative_value_at_end_ptr, 
                             uintptr_t len_derivative_equality_at_start, 
                             uintptr_t len_derivative_equality_at_end);

    double spline_evaluate(const SplineHandle *handle, double x);

    double spline_evaluate_derivative(const SplineHandle *handle, double x, uint8_t order);

    void spline_free(SplineHandle *handle);

}  // extern "C"
#include <eigen3/Eigen/Eigen>
class Spline {
    public:
        Spline(const Eigen::VectorXd &x, const Eigen::VectorXd &y, uint8_t order, 
               const Eigen::VectorXd &derivative_equality_at_start, const Eigen::VectorXd &derivative_value_at_start, 
               const Eigen::VectorXd &derivative_equality_at_end, const Eigen::VectorXd &derivative_value_at_end);
        Spline(const Eigen::VectorXd &x, const Eigen::VectorXd &y, uint8_t order);
        ~Spline();
        double evaluate(double x) const;
        double evaluate_derivative(double x, uint8_t order) const;
    private:
        SplineHandle *handle_;
};