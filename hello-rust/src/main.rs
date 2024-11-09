#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    King, 
    Queen,
    Bishop,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}

type Field = Option<Piece>;

struct ChessBoard {
    fields: [[Field; 8]; 8],
}

impl ChessBoard {
    fn empty() -> ChessBoard {
        ChessBoard{ fields: [[Option::None; 8]; 8] }
    }
}

fn f(piece: Piece) {
    let text: &str = match piece {
        Piece::White(PieceType::King) => "White king",
        _ => "Unknown (probably black, are ya missing anything)",
    };

    println!("{:?}", text);
}


fn main() {
    println!("Hello, world!");

    let p2: Piece = Piece::Black(PieceType::King);

    println!("{:?}", p2);

    let board = ChessBoard::empty();
    f(p2);
}
