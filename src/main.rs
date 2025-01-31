use std::env;

use adding::add;
use divide::div;
use factorial::fact;
use modulus::modulus;
use multiply::mult;
use exponent::exp;
use subtracting::sub;
mod divide;
mod exponent;
mod factorial;
pub mod modulus;
mod multiply;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _len = args.len();

    let x = &args[1];
    let opt = &args[2];
    let opt: char = opt.trim().parse().expect("Error");
    let y = &args[3];

    let x: f64 = x.trim().parse().expect("Enter a number");
    let y: f64 = y.trim().parse().expect("Enter a number");


  
    match opt {
        '+' => println!("Result of {x} + {y} = {}", add(x, y)),
        '-' => println!("Result of {x} - {y} = {}", sub(x, y)),
        'x' => println!("Result of {x} x {y} = {}", mult(x, y)),
        '/' => println!("Result of {x} / {y} = {}", div(x, y)),
        '%' => println!("Result of {x} % {y} = {}", modulus(x as u128, y as u128)),
        '!' => println!("Result of {}! = {}", x, fact(x as u128, x as u128)),
        '^' => println!("Result of {} ^ {} = {}",x,y,exp(x,y)),

        _ => println!("Please Enter either +,  -, x, /, %"),
    };

}

mod adding;
mod subtracting;
