pub fn fact(x: u128, _y: u128) -> u128
{
    let mut fact = 1;
    let mut i  = 1;
    if x == 0
    {
        1
    }
    else
    { 
        while i <= x
        {
              fact = fact * i;
            i = i + 1;
        }
        fact 
    }
}