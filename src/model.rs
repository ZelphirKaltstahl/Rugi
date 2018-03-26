use std::fmt;

pub struct Square {
    content: Option<Piece>
}
pub struct Board {
    squares: Vec<Vec<Square>>
}

pub enum Player {
    Sente,
    Gote
}

pub struct Piece {
    pub kind: PieceKind,
    pub owner: Player,
    pub promoted: bool
}

pub enum PieceKind {
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
    KingSente,
    KingGote
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let owner_prefix = format!("{}", self.owner);
        let promotion_prefix = match self.promoted {
            true => "+",
            false => ""
        };
        let piece_str = format!("{}", self.kind);
        write!(f, "{}{}{}",
               owner_prefix,
               promotion_prefix,
               piece_str)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player::Sente => write!(formatter, "☗"),
            //&Player::Sente => write!(formatter, "B"),
            &Player::Gote => write!(formatter, "☖")
            //&Player::Gote => write!(formatter, "W")
        }
    }
}

impl fmt::Display for PieceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let piece_str = match self {
            &PieceKind::Pawn => "P",
            &PieceKind::Lance => "L",
            &PieceKind::Knight => "N",
            &PieceKind::Silver => "S",
            &PieceKind::Gold => "G",
            &PieceKind::Bishop => "B",
            &PieceKind::Rook => "R",
            &PieceKind::KingSente => "K",
            &PieceKind::KingGote => "K"
        };
        write!(formatter, "{}", piece_str)
    }
}
