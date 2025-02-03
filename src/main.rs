mod calculator;
mod readnum;
use readnum::read_num;
use calculator::Calculator;

fn main() {

    let opt = read_num("Please enter the operator: ");
    let opt: char = opt.trim().parse().expect("Enter a char");
    if opt == '!'
    {
        let num1 = read_num("Enter a number");
        let num1: f64 = num1.trim().parse().expect("Expected a number ");
        let cal = Calculator::new(num1,0_f64);
        println!("{}! = {}",num1,cal.fact());
    }
    else if opt == '+'
    {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.add());
    }
    else if opt == '^'
    {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.pow());
    }
    else if opt == '/' {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.div());       
    }
    else if opt == '*' {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.mult()); 
    }
    else if opt == '%' {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.modulus()); 
    }
    else if opt == '~' {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.expo()); 
    }
    else if opt == '-' {
        let (num1, num2) = get_nums();
        let cal = Calculator::new(num1, num2);
        println!("{} {} {} = {}",num1,opt,num2,cal.subtract()); 
    }
    else {
        println!("Please enter one of the following operators +, -, *, /, %, ~, !, ^,");
    }
}

fn get_nums() -> (f64, f64)
{
    let num1 = read_num("Please Enter the first number");
    let num1: f64 = num1.trim().parse().expect("Enter a number");
    let num2 = read_num("Please enter the second number ");
    let num2: f64 = num2.trim().parse().expect("Enter a number");
    (num1 , num2)
}































































































































// use adding::add;
// use divide::div;
// use power::exp;
// use exponent::expo;
// use factorial::fact;
// use modulus::modulus;
// /*
// use readnum::read_num;
// */
// use multiply::mult;
// use std::{
//     fs::{File, OpenOptions},
//     io::{self, Read, Write},
// };
// use subtracting::sub;
// mod readnum;
// mod exponent;
// mod divide;
// mod power;
// mod factorial;
// pub mod modulus;
// mod multiply;

// fn main() {
   
//    // taking the first number
    
    

//     // taking the operator
//     let opt = read_num("Enter the operator");
//     let opt:char = opt.trim().parse().expect("Please enter a character");
//     if opt == '+'||opt =='-'|| opt == '/'|| opt == 'x'||opt == '%'||opt == '^'
//     {
//         let num =read_num("Please Enter a number");  
//         let num:f64 = num.trim().parse().expect("Enter a number");
//         let num2 =read_num("Please Enter a number");
//         let num2:f64 = num2.trim().parse().expect("Enter a number");

//     match opt {
//         '+' => {
//             println!("Result of {num} + {num2} = {}", add(num, num2));
//             history(num, opt, num2, add(num, num2));
//         }
//         '-' => {
//             println!("Result of {num} - {num2} = {}", sub(num, num2));
//             history(num, opt, num2, sub(num, num2));
//         }
//         'x' => {
//             println!("Result of {num} x {num2} = {}", mult(num, num2));
//             history(num, opt, num2, mult(num, num2));
//         }
//         '/' => {
//             println!("Result of {num} / {num2} = {}", div(num, num2));
//             history(num, opt, num2, div(num, num2));
//         }
//         '%' => {
//             println!(
//                 "Result of {num} % {num2} = {}",
//                 modulus(num as u128, num2 as u128)
//             );
//             let ans = modulus(num as u128, num2 as u128);
//             history(num, opt, num2, ans as f64);
//         }
//         '^' => {
//             println!("Result of {} ^ {} = {}", num, num2, exp(num, num2));

//             history(num, opt, num2, exp(num, num2));
//         }
//         '~' =>{
//             println!("Result of {num} exp {num2} = {}",expo(num, num2));
//         }
        
//         _ =>{ println!("Please Enter either +,  -, x, /, %, ~");
// 		_read_history();
// 		}
//     };

// }
// else if opt == '!' {
//         let num =read_num("Please Enter a number");  
//         let num:f64 = num.trim().parse().expect("Enter a number");
//         println!("Result of {}! = {}", num, fact(num as u128));
//         let ans = fact(num as u128);
//         history(num, opt, num,ans as f64);
// }
//    println!("\n");
//     // let message ="Hello welcome to my calculator program";
//     // let mess2 ="My name is richmond";
//     // fs::write("./history",message);
//     // fs::write("./history",mess2);
//     // let output = fs::read_to_string("./history").expect("Nothing to read");
//     // println!("{}",output);
// }

// // Function to log history
// fn history(num: f64, opt: char, num2: f64, result: f64) {
//     let history = format!("{} {} {} = {}\n", num, opt, num2, result);
//     let mut file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("history.txt")
//         .expect("Cannot open file");
//     file.write_all(history.as_bytes())
//         .expect("Failed to write to file");
// }

// fn read_num(message: &str) -> String
// {
//     println!("Hello {}", message);
//     let mut num = String::new();
//     io::stdin().read_line(&mut num).expect("Enter a number");
//     num

// }
// fn _read_history()  {
// let mut file  = File::open("./history.txt").expect("Unable to read file");

// let mut content = String::new();

// file.read_to_string(&mut content).expect("Unable to read file");

// println!("{}", content);
//     // println!("{}", {history.txt});
// }

// /*This function is used to read a number */

// mod adding;
// mod subtracting;
// /*


//     let args: Vec<String> = env::args().collect();


//     let _len = args.len();

//     let x = &args[1];
//     let opt = &args[2];
//     let opt: char = opt.trim().parse().expect("Error");
//     let y = &args[3];

//     let x: f64 = x.trim().parse().expect("Enter a number");
//     let y: f64 = y.trim().parse().expect("Enter a number");




// */
