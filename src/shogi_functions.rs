use shogi::{Move, Position, MoveRecord, SfenError};
// use shogi::square::consts::*;

pub fn is_valid_move_in_position(mut position: Position, move_in_position: Move) -> bool {
    match position.make_move(&move_in_position) {
        Err(_move_err) => {
            println!("Position as SFEN: {}", position.to_sfen());
            println!("Move: {}", move_in_position);
            println!("The move is not valid in the position.");
            false
        },
        Ok(()) => {
            println!("The move is valid in the position.");
            true
        }
    }
}

pub fn is_valid_move_after_move_record(move_record: MoveRecord, move_in_position: Move) -> Result<bool, SfenError> {
    let mut pos = Position::new();
    match pos.set_sfen(&move_record.to_sfen()) {
        Err(sfen_error) => Err(sfen_error),
        Ok(()) => Ok(is_valid_move_in_position(pos, move_in_position))
    }
}
