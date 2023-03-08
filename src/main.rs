mod moves;
mod pieces;
mod uci;
mod search;

use std::process::exit;
use board::Board;
use fen::build_board;
use pieces::Piece;
use search::{search_moves, perft};
use crate::{moves::{check_check, generate_all_moves}, search::time_move_search};

mod board;
mod fen;

fn main() {
    let board = build_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
    // Current problem child is position fen r3k2r/Pppp2pp/3N1nb1/bP6/B1P1P3/q4N2/Pp1P2PP/R2Q1RK1 b kq - 0 1
    uci::main_loop();
}

#[allow(dead_code)]
fn print_moves(board: &Board) {
    println!("{}", board);
    let mut board = *board;
    let mut moves = generate_all_moves(&board);
    let i = moves.len();
    check_check(&mut board, &mut moves);
    for m in moves.iter() {
        println!("{}", m);
        let mut cloned_board = board;
        cloned_board.make_move(m);
        println!("{}", cloned_board);
        println!("---------------------------------------------------------");
    }
    println!("{} moves possible pre check", i);
    println!("{} moves possible post check", moves.len());
    exit(0);
}

#[cfg(test)]
mod move_number_tests {
    use crate::{search::perft, fen::{build_board, self}};

    #[test]
    fn test_position_3() {
        let board = build_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
        assert_eq!(11030083, perft(&board, 6));
    }

    #[test]
    fn test_starting_pos() {
        let board = build_board(fen::STARTING_FEN);
        assert_eq!(119060324, perft(&board, 6));
    }
}
