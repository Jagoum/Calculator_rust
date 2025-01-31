pub fn fact(x: u128) -> u128
{
    let mut fact = 1;
    let i  = 1;

    if x == 0{
        1
    }
    else
    {
        
        while i != x
        {
              fact = fact * i;
             if i == x {
              break;
             }
        }

        fact
        
    }
}