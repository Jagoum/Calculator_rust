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
   
   // taking the first number
   println!("Please enter the first number : ");
   let mut num = String::new();
   io::stdin().read_line(&mut num).expect("Enter a number");
   let num: f64 = match num.trim().parse() {
       Ok(num) => num,
       Err(_) => {
           print!("Enter number1 ");
           return;
       }
   };

    // taking the operator
    println!("Please enter the operator : ");
    let mut opt = String::new();

    io::stdin().read_line(&mut opt).expect("Enter a number");
    let opt: char = match opt.trim().parse() {
        Ok(opt) => opt,
        Err(_) => {
            print!("Enter operator");
            return;
        }
    };

      // taking the second number

      println!("Please enter the second number : ");
      let mut num2 = String::new();
      io::stdin().read_line(&mut num2).expect("Enter a number");
      let num2: f64 = match num2.trim().parse() {
          Ok(num2) => num2,
          Err(_) => {
              print!("Enter number1 ");
              return;
          }
      };
       

    match opt {
        '+' => {
            println!("Result of {num} + {num2} = {}", add(num, num2));
            history(num, opt, num2, add(num, num2));
        }
        '-' => {
            println!("Result of {num} - {num2} = {}", sub(num, num2));
            history(num, opt, num2, sub(num, num2));
        }
        'x' => {
            println!("Result of {num} x {num2} = {}", mult(num, num2));
            history(num, opt, num2, mult(num, num2));
        }
        '/' => {
            println!("Result of {num} / {num2} = {}", div(num, num2));
            history(num, opt, num2, div(num, num2));
        }
        '%' => {
            println!(
                "Result of {num} % {num2} = {}",
                modulus(num as u128, num2 as u128)
            );
            let ans = modulus(num as u128, num2 as u128);
            history(num, opt, num2, ans as f64);
        }
        '!' => {
            println!("Result of {}! = {}", num, fact(num as u128, num as u128));
            let ans = fact(num as u128, num as u128);
            history(num, opt, num2, ans as f64);
        }
        '^' => {
            println!("Result of {} ^ {} = {}", num, num2, exp(num, num2));

            history(num, opt, num2, exp(num, num2));
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
fn history(num: f64, opt: char, num2: f64, result: f64) {
    let history = format!("{} {} {} = {}\n", num, opt, num2, result);
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
    println!("{:?}", &file);
}

/*This function is used to read a number */

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
