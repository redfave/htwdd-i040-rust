use std::ops::{Mul, Neg, Add};

use crate::NumberType;

#[derive(Debug, Copy, Clone)]
pub struct ComplexNumber {
    real_part: NumberType,
    imaginary_part: NumberType,
}

impl ComplexNumber {
    pub fn new(real_part: NumberType, imaginary_part: NumberType) -> Self {
        Self {
            real_part,
            imaginary_part,
        }
    }
}

impl Neg for ComplexNumber {
    type Output = ComplexNumber;
    fn neg(self) -> Self::Output {
        ComplexNumber::new(-self.real_part, -self.imaginary_part)
    }
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber::new(self.real_part + rhs.real_part, self.imaginary_part + rhs.imaginary_part)
    }
    
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;
    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber::new(self.real_part * rhs.real_part, 
            self.imaginary_part * rhs.imaginary_part)
    }    
}




