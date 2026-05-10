#pragma once
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct SplineHandle;

extern "C" {

SplineHandle *spline_new(uint8_t order, const double *x_ptr, const double *y_ptr, uintptr_t len);

double spline_evaluate(const SplineHandle *handle, double x);

double spline_evaluate_derivative(const SplineHandle *handle, double x, uint8_t order);

void spline_free(SplineHandle *handle);

}  // extern "C"
