
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

fn param_of_kth_derivative_of_polynomial(order: usize, k: usize) -> f64 {
    if k > order {
        0.0
    } else {
        factorial(order) as f64 / factorial(order - k) as f64
    }
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
        let mut solve_matrix = Mat::<f64>::zeros(n*m, n*m);
        let mut b = Col::<f64>::zeros(n*m);

        // the spline equation at the same interval holds this way:
        // S = a0 + a1*(x-x0) + a2*(x-x0)^2 + ... + a_order*(x-x0)^order

        // the n-order derivative at 0 and at the end of the interval is 0, which means:
        // There need to be order-1 addtional equations to do the linear equation solving, which means the total number of equations is n*m, which is the same as the number of unknowns (the params matrix has n*m elements).
        let mut equation_index = 0;
        if order != 1 {

            let order_end = (order as usize - 1) / 2;
            for i in 0 .. order_end {
                // at the beginning of the interval, the order_start to n-order derivative is 0
                
                solve_matrix[(equation_index, i+1)] = factorial(i+1) as f64;
                b[equation_index] = 0.0;
                equation_index += 1;
                for j in i..(order as usize) {
                    print!("j: {}, and {}th derivative of polynomial: {}\n", j+1, i+1, param_of_kth_derivative_of_polynomial(j+1, i+1));
                    solve_matrix[(equation_index, (m-1)*n + j+1)] = param_of_kth_derivative_of_polynomial(j+1, i+1) * (x[m] - x[m-1]).powi((j - i) as i32);
                }
                
                b[equation_index] = 0.0;
                equation_index += 1;
            }
            if (order as usize - 1) % 2 != 0 {
                // if order is odd, the order_end+1 derivative at the beginning is 0
                solve_matrix[(equation_index, order_end+1)] = factorial(order_end+1) as f64;
                b[equation_index] = 0.0;
                equation_index += 1;
            }

        }
        // at each x[i], the spline equation holds, which means:
        for i in 0..m {
            // at x[i], the constant term is y[i]
            solve_matrix[(equation_index, i * n)] = 1.0;
            b[equation_index] = y[i];
            equation_index += 1;
            // at x[i+1], the equation equals y[i+1]
            for j in 0..n {
                solve_matrix[(equation_index, i * n + j)] = (x[i + 1] - x[i]).powi(j as i32);
            }
            b[equation_index] = y[i + 1];
            equation_index += 1;
            if i != 0 {
                // at x[i], S^(1..order)_{i-1} = S^(1..order)_{i}
                for j in 1..(order as usize) {
                    // S^(1..order)_{i}
                    solve_matrix[(equation_index, i * n + j)] = factorial(j) as f64;
                    // S^(1..order)_{i-1}
                    for k in j..(order as usize+1) {
                        // print!("k: {}, and {}th derivative of polynomial: {}\n", k, j, param_of_kth_derivative_of_polynomial(k, j));
                        solve_matrix[(equation_index, (i - 1) * n + k)] = -param_of_kth_derivative_of_polynomial(k, j)*(x[i] - x[i - 1]).powi((k - j) as i32);
                    }
                    b[equation_index] = 0.0;
                    equation_index += 1;
                } 
            } 
        }
        print!("solve_matrix:{:?}\n", solve_matrix);
        // solve the linear equation solve_matrix*x = b
        let flu = solve_matrix.full_piv_lu();
        let result = flu.solve(&b);
        let params = Mat::from_fn(n, m, |i, j| result[j*n+i]);
        // print!("params: {:?}\n", solve_matrix*result - b);
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
                if i != self.x.len() - 1 {
                    // if x is not the last data point, we can use the params of the interval to compute the derivative
                    for j in (order as usize)..(self.order as usize + 1) {
                        result += self.params[(j, i)] * param_of_kth_derivative_of_polynomial(j, order as usize) * (x - self.x[i]).powi((j - order as usize) as i32);
                    }
                }
                else {
                    // if x is the last data point, we can use the params of the last interval to compute the derivative
                    for j in (order as usize)..(self.order as usize + 1) {
                        result += self.params[(j, i - 1)] * param_of_kth_derivative_of_polynomial(j, order as usize) * (x - self.x[i - 1]).powi((j - order as usize) as i32);
                    }
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
                    result += self.params[(j, interval_index)] * param_of_kth_derivative_of_polynomial(j, order as usize) * (x - self.x[interval_index]).powi((j - order as usize) as i32);
                }
                result
            }
        }
    }

}
