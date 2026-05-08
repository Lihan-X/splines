
use faer::{Mat, Col};
use faer::prelude::*;
pub struct Spline{
    order: u8,
    x: Vec<f64>,
    y: Vec<f64>,
    params: Mat<f64>, // params is the size of (order+1, n-1), where n is the number of points
}
fn factorial(n: usize) -> usize {
    (1..=n).product()
}
impl Spline{
    pub fn new(order: u8, x: Vec<f64>, y: Vec<f64>) -> Self{
        // check if x and y have the same length
        if x.len() != y.len(){
            panic!("x and y must have the same length");
        }

        // check if x is sorted from small to big
        if x.windows(2).any(|w| w[0] > w[1]) {
            panic!("x must be sorted from small to big");
        }

        
        // compute the params matrix
        // solve the linear equation 
        let n = order as usize + 1; // number of intervals
        let m = x.len() - 1; // number of intervals

        // initialize params matrix
        let mut A = Mat::<f64>::zeros(n*m, n*m);
        let mut b = Col::<f64>::zeros(n*m);

        // the spline equation at the same interval holds this way:
        // S = a0 + a1*(x-x0) + a2*(x-x0)^2 + ... + a_order*(x-x0)^order

        // the n-order derivative at 0 and at the end of the interval is 0, which means:
        // at 0
        let mut equation_index = 0;
        A[(equation_index, (n - 1))] = 1.0;
        b[equation_index] = 0.0;
        equation_index += 1;
        // at the end of the interval
        A[(equation_index, m * n - 1)] = (factorial(order as usize) as f64)*(x[x.len() - 1] - x[x.len() - 2]);
        A[(equation_index, m * n - 2)] = 1.0;
        b[equation_index] = 0.0;
        equation_index += 1;

        // at each x[i], the spline equation holds, which means:
        for i in 0..m {
            // at x[i], the constant term is y[i]
            A[(equation_index, i * n)] = 1.0;
            b[equation_index] = y[i];
            equation_index += 1;
            // at x[i+1], the equation equals y[i+1]
            for j in 0..n {
                A[(equation_index, i * n + j)] = (x[i + 1] - x[i]).powi(j as i32);
            }
            b[equation_index] = y[i + 1];
            equation_index += 1;
            if i != 0 {
                // at x[i], S^(1..n)_{i-1} = S^(1..n)_{i}
                for j in 1..(order as usize) {
                    // S^(1..n)_{i}
                    A[(equation_index, i * n + j)] = factorial(j) as f64;
                    // S^(1..n)_{i-1}
                    for k in j..(order as usize) { 
                        A[(equation_index, (i - 1) * n + k + 1)] = -(factorial(k) as f64)*(x[i] - x[i - 1]).powi((k - j) as i32);
                    }
                    b[equation_index] = 0.0;
                    equation_index += 1;
                } 
            }
        }
        // solve the linear equation Ax = b
        let flu = A.full_piv_lu();
        let result = flu.solve(&b);
        let params = Mat::from_fn(n, m, |i, j| result[j*n+i]);
        Self { order, x, y, params }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        // find the interval that x belongs to
        match self.x.binary_search_by(|probe| probe.partial_cmp(&x).unwrap()) {
            Ok(i) => {
                // x is exactly at a data point
                self.y[i]
            }
            Err(i) if i == 0 => {
                panic!("x is before the first data point");
            }
            Err(i) if i == self.x.len() => {
                // x is after the last data point
                panic!("x is after the last data point");
            }
            Err(i) => {
                // x is between two data points
                let interval_index = i - 1;
                let mut result = 0.0;
                for j in 0..(self.order as usize + 1) {
                    result += self.params[(j, interval_index)] * (x - self.x[interval_index]).powi(j as i32);
                }
                result
            }
        }
    }

    pub fn evaluate_derivative(&self, x: f64, order: u8) -> f64 {
        // find the interval that x belongs to
        if order > self.order {
            panic!("derivative order must be less than or equal to the spline order");
        }
        match self.x.binary_search_by(|probe| probe.partial_cmp(&x).unwrap()) {
            Ok(i) => {
                // x is exactly at a data point
                let mut result = 0.0;
                for j in (order as usize)..(self.order as usize + 1) {
                    result += self.params[(j, i)] * factorial(j) as f64;
                }
                result
            }
            Err(i) if i == 0 => {
                panic!("x is before the first data point");
            }
            Err(i) if i == self.x.len() => {
                // x is after the last data point
                panic!("x is after the last data point");
            }
            Err(i) => {
                // x is between two data points
                let interval_index = i - 1;
                let mut result = 0.0;
                for j in (order as usize)..(self.order as usize + 1) {
                    result += self.params[(j, interval_index)] * factorial(j) as f64 * (x - self.x[interval_index]).powi((j - order as usize) as i32);
                }
                result
            }
        }
    }

}
