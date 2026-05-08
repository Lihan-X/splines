
mod splines;
use splines::spline::Spline;

fn main() {
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let y = vec![0.0, 1.0, 4.0, 9.0];
    let spline = Spline::new(4, x, y);
    let x_test = 0.9;
    println!("{}", spline.evaluate(x_test));
    println!("{}", spline.evaluate_derivative(x_test, 1));
    println!("{}", spline.evaluate_derivative(x_test, 2));
    println!("{}", spline.evaluate_derivative(x_test, 3));
}
