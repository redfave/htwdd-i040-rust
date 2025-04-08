use rust_prak05_lib::ComplexNumber;
use rust_prak05_lib::NumberType;
use rust_prak05_lib::ChessFigure;
use rust_prak05_lib::ValueType;

fn main() {
    let z_1 =
        ComplexNumber::new(NumberType::Int(-2), NumberType::Float(5.5));
    println!("Initial: {:?}", z_1);

    let z_1_neg = -z_1;
    println!("Negation {:?}", z_1_neg);

    let z_2 = ComplexNumber::new(NumberType::Int(4), NumberType::Int(6));
       
     println!("Operand 1: {:?}", z_1);
    println!("Operand 2: {:?}", z_2);

    let z_3 = z_1 + z_2;
    println!("Addition result: {:?}", z_3);

    let z_4 = z_1 * z_2;
    println!("Multiplication result: {:?}", z_4);



    let king = ChessFigure::King;
    let king_value_classic = king.figure_value(ValueType::Classic);
    println!("King has value of {king_value_classic}");

}
