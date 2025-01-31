pub fn modulus(x: u128, y: u128) -> u128
{
    if y == 0
    {
        print!("Error");
        1
    }   
    else 
    {
        x%y
    }
}