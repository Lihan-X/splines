
pub mod splines;
#[cfg(test)]
mod tests {
    // use super::*;
    
    use crate::splines::spline::Spline;
    #[test]
    fn test_cubic_spline() {
        let x: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y = x.iter().map(|&x| x.powi(3)).collect::<Vec<f64>>();

        let spline = Spline::new(3, x, y);

        let tol = 1e-6;
        let test_x = 1.5;

        // f(x) = x^3
        let expected = (test_x as f64).powi(3);
        let val = spline.evaluate(test_x);
        assert!(
            (val - expected).abs() < tol,
            "evaluate failed: expected {}, got {}",
            expected,
            val
        );

        // f'(x) = 3x^2
        let expected_d1 = 3.0 * (test_x as f64).powi(2);
        let d1 = spline.evaluate_derivative(test_x, 1);
        assert!(
            (d1 - expected_d1).abs() < tol,
            "1st derivative failed: expected {}, got {}",
            expected_d1,
            d1
        );

        // f''(x) = 6x
        let expected_d2 = 6.0 * (test_x as f64);
        let d2 = spline.evaluate_derivative(test_x, 2);
        assert!(
            (d2 - expected_d2).abs() < tol,
            "2nd derivative failed: expected {}, got {}",
            expected_d2,
            d2
        );

        // f'''(x) = 6
        let expected_d3 = 6.0;
        let d3 = spline.evaluate_derivative(test_x, 3);
        assert!(
            (d3 - expected_d3).abs() < tol,
            "3rd derivative failed: expected {}, got {}",
            expected_d3,
            d3
        );

    }
}
