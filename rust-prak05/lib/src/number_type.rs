
use std::ops::{Mul, Neg, Add};

#[derive(Debug, Copy, Clone)]
pub enum NumberType {
    Int(i64),
    Float(f64),
}

impl Neg for NumberType {
    type Output = NumberType;
    fn neg(self) -> Self::Output {
        match self {
            NumberType::Int(i) => NumberType::Int(-i),
            NumberType::Float(f) => NumberType::Float(-f),
        }
    }
}

impl Add for NumberType {
    type Output = NumberType;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberType::Int(a), NumberType::Int(b)) => NumberType::Int(a + b),
            (NumberType::Float(a), NumberType::Float(b)) => NumberType::Float(a + b),
            (NumberType::Int(a), NumberType::Float(b)) => NumberType::Float(a as f64 + b),
            (NumberType::Float(a), NumberType::Int(b)) => NumberType::Float(a + b as f64)
            
        }
        
    }
    
}

impl Mul for NumberType {
    type Output = NumberType;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberType::Int(a), NumberType::Int(b)) => NumberType::Int(a * b),
            (NumberType::Float(a), NumberType::Float(b)) => NumberType::Float(a * b),
            (NumberType::Int(a), NumberType::Float(b)) => NumberType::Float(a as f64 * b),
            (NumberType::Float(a), NumberType::Int(b)) => NumberType::Float(a * b as f64)
        }
    }
}