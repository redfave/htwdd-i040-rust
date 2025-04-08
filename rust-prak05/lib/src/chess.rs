#[derive(Debug, Clone, Copy)]
pub enum ChessFigure {
    Queen,
    King,
    Pawn,
    Rook,
    Bishop,
    Knight,
}

impl ChessFigure {
   pub fn figure_value(&self, value_type: ValueType) -> f32 {
        match self {
            ChessFigure::Queen => 9.0,
            ChessFigure::King => f32::INFINITY,
            ChessFigure::Pawn => 1.0,
            ChessFigure::Rook => {
                if value_type == ValueType::Classic {
                    5.0
                } else {
                    4.5
                }
            }
            ChessFigure::Bishop | ChessFigure::Knight => 3.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Classic,
    Modern,
}
