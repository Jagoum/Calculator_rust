use std::io;
pub fn read_num(message: &str) -> String
{
    println!("{} ",message);
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Enter a number");
    num

}