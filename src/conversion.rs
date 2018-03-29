use errors::{ShogiToShogiRsConversionError};
use model::*;

use shogi::moves::{Move};
use shogi::square::{Square};
use shogi::piece::{Piece};
use shogi::piece_type::{PieceType};
use shogi::color::{Color};

pub fn shogi_rs_move_from_shogi_move(shogi_move: ShogiMove) -> Result<Move, ShogiToShogiRsConversionError> {
    match shogi_move {
        // 1. level: match a normal move
        ShogiMove::Normal{from, to, promote} => match shogi_rs_square_from_shogi_square(from) {
            // 2. level: safe handling of "from square"
            Ok(from_square) => match shogi_rs_square_from_shogi_square(to) {
                // 3. level: safe handling of "to square"
                Ok(to_square) => Ok(Move::Normal{from: from_square, to: to_square, promote: promote}),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        },
        // 1. level: match a drop move
        ShogiMove::Drop{to, piece} => match shogi_rs_square_from_shogi_square(to) {
            // 2. level: safe handling of "to square"
            Ok(to_square) => match shogi_rs_piece_type_from_shogi_piece(piece) {
                // 3. level: safe handling of piece
                Ok(shogi_rs_piece_type) => Ok(Move::Drop{to: to_square, piece_type: shogi_rs_piece_type}),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}

pub fn shogi_rs_piece_type_from_shogi_piece(shogi_piece: ShogiPiece) ->
    Result<PieceType, ShogiToShogiRsConversionError>
{
    match shogi_piece.kind {
        ShogiPieceKind::Pawn => {
            if shogi_piece.promoted {
                Ok(PieceType::ProPawn)
            } else {
                Ok(PieceType::Pawn)
            }
        },
        ShogiPieceKind::Lance => {
            if shogi_piece.promoted {
                Ok(PieceType::ProLance)
            } else {
                Ok(PieceType::Lance)
            }
        },
        ShogiPieceKind::Knight => {
            if shogi_piece.promoted {
                Ok(PieceType::ProKnight)
            } else {
                Ok(PieceType::Knight)
            }
        },
        ShogiPieceKind::Silver => {
            if shogi_piece.promoted {
                Ok(PieceType::ProSilver)
            } else {
                Ok(PieceType::Silver)
            }
        },
        ShogiPieceKind::Gold => {
            Ok(PieceType::Gold)
        },
        ShogiPieceKind::Bishop => {
            if shogi_piece.promoted {
                Ok(PieceType::ProBishop)
            } else {
                Ok(PieceType::Bishop)
            }
        },
        ShogiPieceKind::Rook => {
            if shogi_piece.promoted {
                Ok(PieceType::ProRook)
            } else {
                Ok(PieceType::Rook)
            }
        },
        ShogiPieceKind::KingSente => {
            Ok(PieceType::King)
        },
        ShogiPieceKind::KingGote => {
            Ok(PieceType::King)
        }
    }
}

// fn shogi_rs_piece_from_shogi_piece(shogi_piece: ShogiPiece) -> Result<Piece, ShogiToShogiRsConversionError> {
//     // todo
// }

pub fn shogi_rs_square_from_shogi_square(shogi_square: ShogiSquare) ->
    Result<Square, ShogiToShogiRsConversionError>
{
    match Square::new(shogi_square.file, shogi_square.rank) {
        Some(shogi_rs_square) => Ok(shogi_rs_square),
        None => Err(ShogiToShogiRsConversionError{})
    }
}
