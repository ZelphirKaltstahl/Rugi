use std::fmt;

// use actix::*;
use actix_web::{Error, Responder, HttpResponse, HttpRequest, Result};
// use actix_web::http::{HttpError};

// use shogi::{
//     Position,
//     MoveRecord,
//     Move,
//     PieceGrid,
//     Hand,
//     Color,
//     Bitboard
// };
use serde_json;
// use serde::{Deserialize, Serialize, Deserializer, Serializer};

// use actix::*;

#[derive(Serialize, Deserialize)]
pub enum GameStatus {
    OnGoing,
    Draw,
    SenteWins,
    SenteLoses,
    GoteWins,
    GoteLoses
}

#[derive(Serialize, Deserialize)]
pub enum Player {
    Sente,
    Gote
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub squares: Vec<Vec<ShogiSquare>>
}

#[derive(Serialize, Deserialize)]
pub struct ShogiSquare {
    pub file: u8,
    pub rank: u8,
    pub piece: Option<ShogiPiece>
}

#[derive(Serialize, Deserialize)]
pub struct ShogiPiece {
    pub kind: ShogiPieceKind,
    pub owner: Player,
    pub promoted: bool
}

#[derive(Serialize, Deserialize)]
pub enum ShogiPieceKind {
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

#[derive(Serialize, Deserialize)]
pub enum ShogiMove {
    Normal {
        from: ShogiSquare,
        to: ShogiSquare,
        promote: bool,
    },
    Drop {
        to: ShogiSquare,
        piece: ShogiPiece,
    },
}

#[derive(Serialize, Deserialize)]
pub struct TryMoveResponse {
    pub is_valid: bool,
    pub game_status: Option<GameStatus>
}

#[derive(Serialize, Deserialize)]
pub struct TryMoveInPositionRequest {
    pub sfen: String,
    pub move_in_position: ShogiMove
}
#[derive(Serialize, Deserialize)]
pub struct TryMoveAfterMoveSequence {
    pub move_sequence: Vec<ShogiMove>,
    pub move_in_position: ShogiMove
}

// =======================
// DISPLAY IMPLEMENTATIONS
// =======================
impl fmt::Display for TryMoveInPositionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{{SFEN: {}, move: {}}}",
            self.sfen, self.move_in_position
        )
    }
}

impl fmt::Display for ShogiMove {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // let from_str: &str = format!("{}", self.from);

        match self {
            &ShogiMove::Normal{ref from, ref to, ref promote} => write!(
                formatter,
                "{{from: {}, to: {}, promote: {}}}",
                from, to, promote
            ),
            &ShogiMove::Drop{ref to, ref piece} => write!(
                formatter,
                "{{to: {}, piece: {}}}",
                to, piece
            )
        }
    }
}

impl fmt::Display for ShogiSquare {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.piece {
            Some(ref piece) => write!(
                formatter,
                "{{rank/row: {}, file/column: {}, piece: {}}}",
                self.rank, self.file, piece
            ),
            None => write!(
                formatter,
                "{{rank/row: {}, file/column: {}}}",
                self.rank, self.file
            )
        }
    }
}

impl fmt::Display for ShogiPiece {
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

impl fmt::Display for ShogiPieceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let piece_str = match self {
            &ShogiPieceKind::Pawn => "P",
            &ShogiPieceKind::Lance => "L",
            &ShogiPieceKind::Knight => "N",
            &ShogiPieceKind::Silver => "S",
            &ShogiPieceKind::Gold => "G",
            &ShogiPieceKind::Bishop => "B",
            &ShogiPieceKind::Rook => "R",
            &ShogiPieceKind::KingSente => "K",
            &ShogiPieceKind::KingGote => "K"
        };
        write!(formatter, "{}", piece_str)
    }
}





// impl<'s> Serialize for TryMoveRequestWithPosition<'s> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer {

//     }
// }

// impl<'de> Deserialize<'de> for TryMoveRequestWithPosition {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: Deserializer<'de> {
//         Ok()
//     }
// }






// =========================
// RESPONDER IMPLEMENTATIONS
// =========================
impl Responder for TryMoveResponse {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self).expect("could not serialize instance");

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)?)
    }
}




// // implementing
// #[derive(Serialize, Deserialize)]
// #[serde(remote = "Position")]
// struct PositionDef {
//     board: PieceGrid,
//     hand: Hand,
//     ply: u16,
//     side_to_move: Color,
//     move_history: Vec<MoveRecord>,
//     sfen_history: Vec<(String, u16)>,
//     occupied_bb: Bitboard,
//     color_bb: [Bitboard; 2],
//     type_bb: [Bitboard; 14],
// }
// // conversion from PositionDef to Position must be specified for serde to work with it
// impl From<PositionDef> for Position {
//     fn from(def: PositionDef) -> Position {
//         let mut pos = Position::new();
//         pos.set_sfen(def.sfen);
//         pos
//     }
// }

// impl Serialize for Position {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer {
//         *self.to_sfen()
//     }
// }

// impl<'de> Deserialize<'de> for Position {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: Deserializer<'de> {
//         let sfen: &str = Deserialize::deserialize(deserializer)?;  // <- this is the unsafe part where we must get a string slice, otherwise it does not work
//         let pos = Position::new();
//         pos.set_sfen(sfen);
//         Ok(pos)
//     }
// }

// // TODO
// impl Serialize for MoveRecord {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer {
//         *self.to_sfen()
//     }
// }

// impl<'de> Deserialize<'de> for MoveRecord {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: Deserializer<'de> {
//         let sfen: &str = Deserialize::deserialize(deserializer)?;  // <- this is the unsafe part where we must get a string slice, otherwise it does not work
//         // How to do this?
//         // Do I need to write a parser?!
//         // Ok(move_record)
//     }
// }

// impl Response







// pub struct Square {
//     content: Option<Piece>
// }
// pub struct Board {
//     squares: Vec<Vec<Square>>
// }

// pub enum Player {
//     Sente,
//     Gote
// }

// pub struct Piece {
//     pub kind: PieceKind,
//     pub owner: Player,
//     pub promoted: bool
// }

// pub enum PieceKind {
//     Pawn,
//     Lance,
//     Knight,
//     Silver,
//     Gold,
//     Bishop,
//     Rook,
//     KingSente,
//     KingGote
// }
