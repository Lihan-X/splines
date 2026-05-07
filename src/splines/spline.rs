
use faer::Mat;
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
        let params = Mat::<f64>::zeros(n, m);
        let mut A = Mat::<f64>::zeros(n*m, n*m);
        let mut b = Vec::<f64>::new();

        // the spline equation at the same interval holds this way:
        // S = a0 + a1*(x-x0) + a2*(x-x0)^2 + ... + a_order*(x-x0)^order

        // the n-order derivative at 0 and at the end of the interval is 0, which means:
        // at 0
        let mut equation_index = 0;
        A[(equation_index, (n - 1))] = 1.0;
        b.push(0.0);
        equation_index += 1;
        // at the end of the interval
        A[(equation_index, m * n - 1)] = (factorial(order as usize) as f64)*(x[x.len() - 1] - x[x.len() - 2]);
        A[(equation_index, m * n - 2)] = 1.0;
        b.push(0.0);
        equation_index += 1;
        // at each x[i], the spline equation holds, which means:
        for i in 0..x.len() {
            // there will be 
            // at x[i], the constant term is y[i]
            A[(equation_index, i * n)] = 1.0;
            b.push(y[i]);
            equation_index += 1;
            // at x[i+1], the equation equals y[i+1]
            for j in 0..n {
                A[(equation_index, i * n + j)] = (x[i + 1] - x[i]).powi(j as i32);
            }
            b.push(y[i + 1]);
            equation_index += 1;
        }

        Self { order, x, y, params }
    }


}
