//! The alpha-beta algorithm.
use board::*;
use super::*;

use super::{ScoringMove, eval_board};


const MAX_PLY: u16 = 5;

pub fn alpha_beta_search(board: &mut Board, mut alpha: i16, beta: i16, depth: u16) -> ScoringMove {
    if depth == 0 {
        return eval_board(board);
    }

    let mut moves = board.generate_scoring_moves();

    if moves.is_empty() {
        if board.in_check() {
            return ScoringMove::blank(-MATE_V);
        } else {
            return ScoringMove::blank(DRAW_V);
        }
    }

    let mut best_move = ScoringMove::blank(alpha);
    for mov in moves.iter_mut() {
        board.apply_move(mov.bit_move);
        mov.score = -alpha_beta_search(board, -beta, -alpha, depth - 1).score;
        board.undo_move();
        if mov.score > alpha {
            alpha = mov.score;
            if alpha >= beta {
                return *mov;
            }
            best_move = *mov;
        }
    }

    best_move
}
