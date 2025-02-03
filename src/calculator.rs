
pub struct Calculator {
    pub num1: f64,
    pub num2: f64,
}
#[allow(dead_code)]
impl Calculator {
    pub fn new(num1: f64, num2: f64) -> Self {
        Self { num1, num2 }
    }

    pub fn add(&self) -> f64 {
        self.num1 + self.num2
    }
    pub fn subtract(&self) -> f64 {
        self.num1 - self.num2
    }
    pub fn mult(&self) -> f64 {
        self.num1 * self.num2
    }
    pub fn div(&self) -> f64 {
        if self.num2 == 0_f64 {
            return -1.0;
        } else {
            self.num1 as f64 / self.num2 as f64
        }
    }
    pub fn pow(&self) -> f64 {
        self.num1.powf(self.num2)
    }
    pub fn fact(&self) -> f64 {
        let mut fact = 1_f64;
        let mut i = 1_f64;
        if self.num1 == 0_f64 {
            1_f64
        } else {
            while i <= self.num1 {
                fact = fact * i;
                i = i + 1_f64;
            }
            fact
        }
    }
    pub fn modulus(&self) -> f64 {
        self.num1%self.num2
    }
    pub fn expo(&self) -> f64
{
    self.num1*10_f64.powf(self.num2)
}
}
