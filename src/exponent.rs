use crate::power::exp;

pub fn expo(x: f64,y:f64 ) -> f64
{
    x*exp(10.0, y)
}