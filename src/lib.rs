pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
mod splines;
#[cfg(test)]
mod tests {
    use super::*;
    
    use splines::spline::Spline;
    #[test]
    fn it_works() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let y = vec![0.0, 1.0, 4.0, 9.0];
        let spline = Spline::new(3, x, y);
        
        // Sample the spline between 0 and 3
        let num_samples = 300;
        
        for i in 0..=num_samples {
            let x_val = (i as f64 / num_samples as f64) * 3.0;
            let y_val = spline.evaluate(x_val);
            let dy_val = spline.evaluate_derivative(x_val, 1);
            let ddy_val = spline.evaluate_derivative(x_val, 3);
            
        }

    }
}
