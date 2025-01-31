use std::{arch::x86_64::_XCR_XFEATURE_ENABLED_MASK, env::{self, args}, fmt::Write};
use adding::add;
use factorial::fact;
use subtracting::sub;
use multiply::mult;
use divide::div;
mod divide;
mod multiply;
mod factorial;
pub mod modulus;

fn main() {
let args: Vec<String> = env::args().collect();

let len = args.len();

let x = &args[1];
let opt=&args[2];
let opt:char = opt.trim().parse().expect("Error");
let y =&args[3];

let x: f64 = x.trim().parse().expect("Enter a number");
let y: f64 = y.trim().parse().expect("Enter a number");
if len == 4 {
match opt
 {
    '+' => println!("Result of {x} + {y} = {}",add(x,y)), 
    '-' => println!("Result of {x} - {y} = {}",sub(x,y)),
    'x' => println!("Result of {x} x {y} = {}",mult(x,y)),
    '/' => println!("Result of {x} / {y} = {}",div(x, y)) ,
    _ => println!("Please Enter either + ,  -, *, /") 
};
}

}

mod subtracting;
mod adding;