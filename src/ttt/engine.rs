use crate::ttt::{
    board::{Board, CENTER, CORNERS, LINES},
    player::Player,
};
use log::trace;

const WIN: i32 = 1_000_000;

#[inline]
pub fn eval(x: u16, o: u16) -> i32 {
    trace!("eval({x}, {o}) called.");
    const BASE: [i32; 4] = [0, 1, 5, WIN];
    const THREAT: i32 = 20;

    const CENTER_W: i32 = 2;
    const CORNER_W: i32 = 1;

    let mut s = 0i32; // score

    // priorize center and corner squares using weights defined above
    s += CENTER_W * ((x & CENTER != 0) as i32) - CENTER_W * ((o & CENTER != 0) as i32);
    s += CORNER_W * ((x & CORNERS).count_ones() as i32)
        - CORNER_W * ((o & CORNERS).count_ones() as i32);

    for &m in &LINES {
        let xc = (x & m).count_ones() as usize;
        let oc = (o & m).count_ones() as usize;

        // inc score if x would be all alone on the line
        if oc == 0 {
            s += BASE[xc] + if xc == 2 { THREAT } else { 0 };
        }

        // decr score if o would be all alone on the line
        if xc == 0 {
            s -= BASE[oc] + if oc == 2 { THREAT } else { 0 };
        }
    }

    s
}

#[allow(dead_code)]
#[inline]
pub fn negamax(board: &Board, depth: u8) -> i32 {
    let mut position = board.clone();

    // check for win
    if let Some(winner) = position.winner() {
        if winner == board.turn {
            return WIN;
        } else {
            return -WIN;
        }
    };

    // check for draw
    if position.empty() == 0 {
        return 0;
    };

    let mut best = i32::MIN;

    if depth == 0 {
        if position.turn == Player::X {
            return position.eval();
        } else {
            return -position.eval();
        }
    } else {
        for sq in position.legal_moves() {
            position.play_move(sq);
            let negmax = -negamax(&position, depth - 1);
            if negmax > best {
                best = negmax;
            }
            position.unplay_move(sq);
        }
    }

    best
}

#[allow(dead_code)]
#[inline]
pub fn negamax_ab(board: &Board, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    let mut position = board.clone();

    // check for win
    if let Some(winner) = position.winner() {
        if winner == board.turn {
            return WIN;
        } else {
            return -WIN;
        }
    };

    // check for draw
    if position.empty() == 0 {
        return 0;
    };

    let mut best = i32::MIN;

    if depth == 0 {
        if position.turn == Player::X {
            return position.eval();
        } else {
            return -position.eval();
        }
    } else {
        for sq in position.legal_moves() {
            position.play_move(sq);
            let score = -negamax_ab(&position, depth - 1, -beta, -alpha);
            best = best.max(score);
            alpha = alpha.max(score);
            position.unplay_move(sq);
            if alpha >= beta {
                break;
            }
        }
    }

    best
}

#[cfg(test)]
mod test {
    use log::debug;

    use super::*;
    use crate::ttt::board::Board;

    #[test]
    fn eval_start() {
        let board = Board::new();
        let score = negamax(&board, 2);
        debug!("score: {score}");
    }
}
