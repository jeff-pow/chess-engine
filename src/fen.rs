use crate::board::Board;
/** File takes a string in Forsyth-Edwards notation and constructs a board state */
use crate::pieces::{Color, Piece, PieceName};

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
#[allow(dead_code)]
pub const ONE_PIECE: &str = "8/8/8/8/8/4p1p1/5P2/8 w KQkq - 0 1";

pub fn build_board(fen_string: &str) -> Board {
    let mut board = Board::new();
    let mut row = 7;
    let pieces: Vec<&str> = fen_string.split(['/', ' ']).collect();
    // FEN strings have 13 entries (if each slash and each space delimit an entry)
    let mut iter = pieces.iter();
    let mut start = 7;
    let end = 0;
    let step: i32 = -1;
    while start >= end {
        // Loop handles reading board part of fen string
        let entry = iter.next().unwrap();
        let mut idx: usize = 0;
        for c in entry.chars() {
            if c.is_ascii_digit() {
                idx += c.to_digit(10).unwrap() as usize;
                continue;
            } else {
                match c {
                    'K' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::King,
                            (row * 8 + idx) as i8,
                        ));
                        board.white_king_square = (row * 8 + idx) as i8;
                    }
                    'Q' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::Queen,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'R' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::Rook,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'N' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::Knight,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'B' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::Bishop,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'P' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::White,
                            PieceName::Pawn,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'k' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::King,
                            (row * 8 + idx) as i8,
                        ));
                        board.black_king_square = (row * 8 + idx) as i8;
                    }
                    'q' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::Queen,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'r' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::Rook,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'b' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::Bishop,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'n' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::Knight,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    'p' => {
                        board.board[row * 8 + idx] = Some(Piece::new(
                            Color::Black,
                            PieceName::Pawn,
                            (row * 8 + idx) as i8,
                        ));
                    }
                    _ => panic!("Unrecognized char {}, board could not be made", c),
                }
            }
            idx += 1;
        }
        start += step;
        row = row.saturating_sub(1);
    }
    // 9th iteration: find who's turn it is to move
    board.to_move = match iter.next().unwrap().chars().next().unwrap() {
        'w' => Color::White,
        'b' => Color::Black,
        _ => panic!("invalid turn"),
    };
    // 10th bucket find who can still castle
    // Order of array is white king castle, white queen castle, black king castle, black queen castle
    for c in iter.next().unwrap().chars() {
        match c {
            'K' => board.white_king_castle = true,
            'Q' => board.white_queen_castle = true,
            'k' => board.black_king_castle = true,
            'q' => board.black_queen_castle = true,
            '-' => (),
            _ => panic!("Unrecognized castle character: {}", c),
        }
    }
    // En passant square: not yet implemented
    let en_passant_letters: Vec<char> = iter.next().unwrap().chars().collect();
    let en_passant_idx = find_en_passant_square(en_passant_letters);
    if let Some(idx) = en_passant_idx {
        board.en_passant_square = idx as i8
    }
    // Half move clock: not yet implemented
    iter.next();
    // Full number of moves in the game: starts from 1 and incremented after black's first move
    iter.next();
    assert_eq!(iter.next(), None);
    board
}

fn find_en_passant_square(vec: Vec<char>) -> Option<u8> {
    if vec[0] == '-' {
        return None;
    }
    // Using base 20 allows program to convert letters directly to numbers instead of matching
    // against letters or some other workaround
    let column = vec[0].to_digit(20).unwrap() - 10;
    let row = (vec[1].to_digit(10).unwrap() - 1) * 8;
    Some((row + column) as u8)
}

#[allow(clippy::ptr_arg)]
pub fn parse_fen_from_buffer(buf: &[&str]) -> String {
    let mut vec = buf.to_owned();
    vec.remove(0);
    vec.remove(0);
    for _ in 6..vec.len() {
        vec.pop();
    }
    vec.join(" ")
}

#[cfg(test)]
mod fen_tests {
    use crate::fen::find_en_passant_square;

    #[test]
    fn test_en_passant_square() {
        assert_eq!(Some(0), find_en_passant_square(vec!['a', '1']));
        assert_eq!(Some(9), find_en_passant_square(vec!['b', '2']));
        assert_eq!(Some(18), find_en_passant_square(vec!['c', '3']));
        assert_eq!(Some(27), find_en_passant_square(vec!['d', '4']));
        assert_eq!(Some(36), find_en_passant_square(vec!['e', '5']));
        assert_eq!(Some(45), find_en_passant_square(vec!['f', '6']));
        assert_eq!(Some(54), find_en_passant_square(vec!['g', '7']));
        assert_eq!(Some(63), find_en_passant_square(vec!['h', '8']));
        assert_eq!(Some(62), find_en_passant_square(vec!['g', '8']));
    }
}
