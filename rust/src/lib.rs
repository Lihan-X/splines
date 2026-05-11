pub mod splines;

use splines::spline::Spline;
use std::ptr;

// #[repr(C)]
pub struct SplineHandle {
    inner: Spline,
}

#[unsafe(no_mangle)]
pub extern "C" fn spline_new(
    order: u8,
    x_ptr: *const f64,
    y_ptr: *const f64,
    len: usize,
    derivative_equality_at_start_ptr: *const f64,
    derivative_value_at_start_ptr: *const f64,
    derivative_equality_at_end_ptr: *const f64,
    derivative_value_at_end_ptr: *const f64,
    len_derivative_equality_at_start: usize,
    len_derivative_equality_at_end: usize
) -> *mut SplineHandle {
    if x_ptr.is_null() || y_ptr.is_null() || len == 0 {
        return ptr::null_mut();
    }
    let xs = unsafe { std::slice::from_raw_parts(x_ptr, len) };
    let ys = unsafe { std::slice::from_raw_parts(y_ptr, len) };
    let derivative_equality_at_start = if derivative_equality_at_start_ptr.is_null() || len_derivative_equality_at_start == 0 {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(derivative_equality_at_start_ptr, len_derivative_equality_at_start) }.to_vec())
    };
    let derivative_equality_at_end = if derivative_equality_at_end_ptr.is_null() || len_derivative_equality_at_end == 0 {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(derivative_equality_at_end_ptr, len_derivative_equality_at_end) }.to_vec())
    };
    let derivative_value_at_start = if derivative_value_at_start_ptr.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(derivative_value_at_start_ptr, len_derivative_equality_at_start) }.to_vec())
    };
    let derivative_value_at_end = if derivative_value_at_end_ptr.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(derivative_value_at_end_ptr, len_derivative_equality_at_end) }.to_vec())
    };

    let spline = Spline::new(order, xs.to_vec(), ys.to_vec(), derivative_equality_at_start, derivative_value_at_start, derivative_equality_at_end, derivative_value_at_end);

    Box::into_raw(Box::new(SplineHandle { inner: spline }))
}

#[unsafe(no_mangle)]
pub extern "C" fn spline_evaluate(
    handle: *const SplineHandle,
    x: f64,
) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }

    let spline = unsafe { &(*handle).inner };
    spline.evaluate(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn spline_evaluate_derivative(
    handle: *const SplineHandle,
    x: f64,
    order: u8,
) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }

    let spline = unsafe { &(*handle).inner };
    spline.evaluate_derivative(x, order)
}

#[unsafe(no_mangle)]
pub extern "C" fn spline_free(handle: *mut SplineHandle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}