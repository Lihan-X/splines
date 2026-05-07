
mod splines;
use splines::spline::Spline;

fn main() {
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let y = vec![0.0, 1.0, 4.0, 9.0];
    let _spline = Spline::new(3, x, y);
    println!("Hello, world!");
}
