
mod splines;
use splines::spline::Spline;
use std::fs::File;
use std::io::Write;

fn main() {
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let y = vec![0.0, 1.0, 4.0, 9.0];
    let spline = Spline::new(3, x, y);
    
    // Sample the spline between 0 and 3
    let num_samples = 300;
    let mut csv_data = String::from("x,spline,derivative_1,derivative_2\n");
    
    for i in 0..=num_samples {
        let x_val = (i as f64 / num_samples as f64) * 3.0;
        let y_val = spline.evaluate(x_val);
        let dy_val = spline.evaluate_derivative(x_val, 1);
        let ddy_val = spline.evaluate_derivative(x_val, 3);
        
        csv_data.push_str(&format!("{},{},{},{}\n", x_val, y_val, dy_val, ddy_val));
    }
    
    // Write to CSV file
    let mut file = File::create("test_utils/spline_samples.csv").expect("Failed to create file");
    file.write_all(csv_data.as_bytes()).expect("Failed to write to file");
    
    println!("Samples written to test_utils/spline_samples.csv");
}
