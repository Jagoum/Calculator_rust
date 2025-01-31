pub fn div(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        print!("Sorry division by zero is not defined ");
        -1.0
    } else {
        x / y
    }
}
