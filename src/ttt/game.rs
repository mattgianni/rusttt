// struct Game;
use crate::cli::Config;
use crate::error::AppError;
use crate::ttt::board::Board;
use crate::ttt::engine::negamax_ab;
use crate::ttt::player::Player;
use log::trace;

use rand::rng;
use rand::seq::IndexedRandom;

#[allow(dead_code)]
fn rando() {
    let mut rng = rng();

    let mut board = Board::new();
    println!("{board}");

    loop {
        let moves: Vec<u8> = board.legal_moves().collect();

        if moves.is_empty() {
            break;
        }

        println!("{moves:?}");

        if let Some(sq) = moves.choose(&mut rng) {
            println!("playing {sq}");
            board.play_move(*sq);
            println!("{board}");
        } else {
            break;
        }
    }
}

pub fn play(cfg: &Config) -> Result<String, AppError> {
    trace!("play({:?}) called.", cfg);

    // rando();
    let mut board = Board::new();
    board.x = 0b100_000_001;
    board.o = 0b000_010_010;
    board.turn = Player::X;

    let (mut alpha, beta, mut best) = (i32::MIN + 1000, i32::MAX - 1000, i32::MIN + 1000);
    for sq in board.legal_moves() {
        board.play_move(sq);

        let score = -negamax_ab(&board, 9, -beta, -alpha);
        best = best.max(score);
        alpha = alpha.max(score);

        println!("{board}Score: {score}");

        board.unplay_move(sq);
    }

    Ok(format!("processed: {:?}", cfg))
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub moves: Vec<u8>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            moves: Vec::<u8>::new(),
        }
    }

    pub fn play_move(&mut self, sq: u8) {
        self.moves.push(sq);
        self.board.play_move(sq);
    }

    pub fn clear(&mut self) {
        self.board.clear();
        self.moves.clear();
    }

    pub fn play_random(&mut self) {
        let mut rng = rng();
        let moves: Vec<u8> = self.board.legal_moves_safe().collect();
        if !moves.is_empty()
            && let Some(sq) = moves.choose(&mut rng)
        {
            self.play_move(*sq);
        } else {
            self.clear();
        }
    }
}
