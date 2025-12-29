// struct Game;
use crate::ttt::board::Board;

use rand::rng;
use rand::seq::IndexedRandom;

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
