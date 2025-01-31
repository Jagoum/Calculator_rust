use adding::add;
use divide::div;
use exponent::exp;
use factorial::fact;
use modulus::modulus;
use multiply::mult;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};
use subtracting::sub;
mod divide;
mod exponent;
mod factorial;
pub mod modulus;
mod multiply;

fn main() {
   
    let num1 = read_num();
    let opt = read_char();
    let num2 = read_num();

    match opt {
        '+' => {
            println!("Result of {num1} + {num2} = {}", add(num1, num2));
            history(num1, opt, num2, add(num1, num2));
        }
        '-' => {
            println!("Result of {num1} - {num2} = {}", sub(num1, num2));
            history(num1, opt, num2, sub(num1, num2));
        }
        'x' => {
            println!("Result of {num1} x {num2} = {}", mult(num1, num2));
            history(num1, opt, num2, mult(num1, num2));
        }
        '/' => {
            println!("Result of {num1} / {num2} = {}", div(num1, num2));
            history(num1, opt, num2, div(num1, num2));
        }
        '%' => {
            println!(
                "Result of {num1} % {num2} = {}",
                modulus(num1 as u128, num2 as u128)
            );
            let ans = modulus(num1 as u128, num2 as u128);
            history(num1, opt, num2, ans as f64);
        }
        '!' => {
            println!("Result of {}! = {}", num1, fact(num1 as u128, num1 as u128));
            let ans = fact(num1 as u128, num1 as u128);
            history(num1, opt, num2, ans as f64);
        }
        '^' => {
            println!("Result of {} ^ {} = {}", num1, num2, exp(num1, num2));

            history(num1, opt, num2, exp(num1, num2));
        }
        _ => println!("Please Enter either +,  -, x, /, %"),
    };
read_history();
    // let message ="Hello welcome to my calculator program";
    // let mess2 ="My name is richmond";
    // fs::write("./history",message);
    // fs::write("./history",mess2);
    // let output = fs::read_to_string("./history").expect("Nothing to read");
    // println!("{}",output);
}

// Function to log history
fn history(num1: f64, opt: char, num2: f64, result: f64) {
    let history = format!("{} {} {} = {}\n", num1, opt, num2, result);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Cannot open file");
    file.write_all(history.as_bytes())
        .expect("Failed to write to file");
}
fn read_history()  {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("history.txt");
    println!("{:?}", file);
}

/*This function is used to read a number */
fn read_num() -> f64 {
    print!("Please enter the first number : ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Enter a number");
    let num: f64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Enter number1 ");
            return 0_f64;
        }
    };
    num
}
fn read_char() -> char {

    print!("Please enter the operator : ");
    let mut opt = String::new();

    io::stdin().read_line(&mut opt).expect("Enter a number");
    let opt: char = match opt.trim().parse() {
        Ok(opt) => opt,
        Err(_) => {
            print!("Enter operator");
            return '0';
        }
    };
    opt
}

mod adding;
mod subtracting;
/*


    let args: Vec<String> = env::args().collect();


    let _len = args.len();

    let x = &args[1];
    let opt = &args[2];
    let opt: char = opt.trim().parse().expect("Error");
    let y = &args[3];

    let x: f64 = x.trim().parse().expect("Enter a number");
    let y: f64 = y.trim().parse().expect("Enter a number");




*/
