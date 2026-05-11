# Splines
This library implements splines of all orders.
## Quick Start
### create spline
To create a simple cubic spline
```cpp
Eigen::Vector<double, 4> x{1.0, 2.0, 3.0, 4.0};
Eigen::Vector<double, 4> y{1.0, 8.0, 27.0, 64.0};

uint8_t order = 3;
Spline spline(x, y, order);
```
Usually, 
the extra equality of a cubic spline $y=S(x)$ is that
$S''(0)=0$ and $S''(end)=0$.
From a engineer's perspective,
it's more important to keep the first derivative (velocity $S'(0)=0$ and $S'(end)=0$) at the start and at the end 0,
which is default configuration of this library.

However, if you want to specify the extra constraints on you own,
you can also do the following:
```cpp
Eigen::Vector<double, 4> x{1.0, 2.0, 3.0, 4.0};
Eigen::Vector<double, 4> y{1.0, 8.0, 27.0, 64.0};

uint8_t order = 3;
Eigen::Vector<double, 1> derivative_equality_at_start{2.0}; // second derivative at the start
Eigen::Vector<double, 1> derivative_value_at_start{0.0};   // is zero
Eigen::Vector<double, 1> derivative_equality_at_end{2.0}; // second derivative at the end
Eigen::Vector<double, 1> derivative_value_at_end{0.0};    // is zero
Spline spline(x, y, order, derivative_equality_at_start, derivative_value_at_start, derivative_equality_at_end, derivative_value_at_end);
```

Do keep in mind if you want to specify the constraints on your own,
the number of constraints need to be specified is:
$$
n = order - 1
$$

### evaluate spline at specific point
```cpp
double x=1.5;
spline.evaluate(x)
```
It's also possible to evaluate the derivatives:
```cpp
double x=1.5;
spline.evaluate_derivative(1.5, static_cast<uint8_t>(1));
```
