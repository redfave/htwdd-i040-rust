use crate::Car;

#[derive(Debug)]
pub enum CarList {
    Cons(Car, Box<CarList>),
    Nil
}