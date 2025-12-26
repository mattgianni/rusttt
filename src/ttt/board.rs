use crate::ttt::bititer::BitIter;
use crate::ttt::engine::eval;
use crate::ttt::player::Player;

use log::trace;
use std::fmt::Display;

pub const LINES: [u16; 8] = [
    0b000_000_111,
    0b000_111_000,
    0b111_000_000, // rows
    0b001_001_001,
    0b010_010_010,
    0b100_100_100, // cols
    0b100_010_001,
    0b001_010_100, // diags
];
pub const CENTER: u16 = 1 << 4;
pub const CORNERS: u16 = (1 << 0) | (1 << 2) | (1 << 6) | (1 << 8);

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub x: u16,
    pub o: u16,
    pub turn: Player,
}

impl Board {
    pub fn new() -> Board {
        trace!("new() called.");
        Board {
            x: 0,
            o: 0,
            turn: Player::X,
        }
    }

    pub fn eval(&self) -> i32 {
        trace!("eval() called.");
        eval(self.x, self.o)
    }

    pub fn get(&self, sq: u8) -> Option<Player> {
        trace!("get({sq}) called.");
        assert!(sq < 9);
        if self.x & (1 << sq) != 0 {
            Some(Player::X)
        } else if self.o & (1 << sq) != 0 {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn set(&mut self, sq: u8, player: Option<Player>) {
        trace!("set({sq}, {player:?}) called.");
        assert!(sq < 9);

        match player {
            Some(Player::X) => self.x |= 1u16 << sq,
            Some(Player::O) => self.o |= 1u16 << sq,
            None => {
                self.x &= !(1u16 << sq);
                self.o &= !(1u16 << sq);
            }
        }
    }

    #[inline]
    pub fn winner(&self) -> Option<Player> {
        trace!("winner() called.");
        for &m in &LINES {
            if self.x & m == m {
                return Some(Player::X);
            }
            if self.o & m == m {
                return Some(Player::O);
            }
        }
        None
    }

    #[inline]
    pub fn play_move(&mut self, sq: u8) {
        trace!("play_move({sq}) called.");
        assert!(sq < 9);
        assert!(self.get(sq) == None);

        self.set(sq, Some(self.turn));
        self.turn = self.turn.other();
    }

    #[inline]
    pub fn unplay_move(&mut self, sq: u8) {
        trace!("unplay_move({sq}) called.");
        assert!(sq < 9);
        assert!(self.get(sq) != None);

        self.set(sq, None);
        self.turn = self.turn.other();
    }

    #[inline]
    pub fn occupied(&self) -> u16 {
        self.x | self.o
    }

    #[inline]
    pub fn empty(&self) -> u16 {
        !self.occupied() & 0x1FF // 9 squares
    }

    #[inline]
    pub fn legal_moves(&self) -> BitIter {
        trace!("legal_moves() called.");
        BitIter { bb: self.empty() }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for sq in 0..9u8 {
            match self.get(sq) {
                Some(player) => write!(f, "{player}")?,
                None => write!(f, "{sq}")?,
            }

            if (sq + 1) % 3 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())

        // writeln!(f, "eval: {}\n", eval(self.x, self.o))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_board_new() {
        let board = Board::new();
        assert_eq!(board.o, 0u16);
        assert_eq!(board.x, 0u16);
        assert_eq!(board.turn, Player::X);
    }

    #[test]
    fn check_board_get() {
        let mut board = Board::new();

        assert_eq!(board.get(4), None);
        board.x = 1 << 4 | 1 | 1 << 8;
        board.o = 1 << 5 | 1 << 3;

        assert_eq!(board.get(4), Some(Player::X));
        assert_eq!(board.get(5), Some(Player::O));
        assert_eq!(board.get(0), Some(Player::X));
        assert_eq!(board.get(7), None);
    }

    #[test]
    fn check_board_set() {
        let mut board = Board::new();

        board.set(4, Some(Player::X));
        board.set(0, Some(Player::X));
        board.set(8, Some(Player::X));

        board.set(5, Some(Player::O));
        board.set(3, Some(Player::O));
        board.set(2, Some(Player::O));
        board.set(2, None);

        assert_eq!(board.x, 1u16 << 4 | 1u16 << 0 | 1u16 << 8);
        assert_eq!(board.o, 1 << 5 | 1 << 3);
    }

    #[test]
    fn check_play_move() {
        let mut board = Board::new();

        board.play_move(4);
        assert_eq!(board.get(4), Some(Player::X));
        assert_eq!(board.x, (1 << 4) as u16);
        assert_eq!(board.o, 0u16);
        assert_eq!(board.turn, Player::O);

        board.play_move(3);
        assert_eq!(board.get(3), Some(Player::O));
        assert_eq!(board.x, (1 << 4) as u16);
        assert_eq!(board.o, (1 << 3) as u16);
        assert_eq!(board.turn, Player::X);
    }

    #[test]
    fn check_board_winner() {
        let mut board = Board::new();

        assert_eq!(board.winner(), None);

        board.play_move(4);
        assert_eq!(board.winner(), None);
        board.play_move(5);
        assert_eq!(board.winner(), None);

        board.play_move(0);
        assert_eq!(board.winner(), None);
        board.play_move(3);
        assert_eq!(board.winner(), None);

        board.play_move(8);
        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn check_eval_minor() {
        let mut board = Board::new();
        board.x = 0b000_000_011;
        board.o = 0b000_010_000;
        assert!(board.eval() > 10); // white should be winning modestly

        board.x = 0b000_000_011;
        board.o = 0b000_010_100;
        assert!(board.eval() < 10); // black should winning slightly
    }

    #[test]
    fn check_eval_major() {
        let mut board = Board::new();
        board.x = 0b001_000_101;
        board.o = 0b100_010_000;
        assert!(board.eval() > 30); // white should be crush
    }

    #[test]
    fn check_eval_win() {
        let mut board = Board::new();
        board.x = 0b001_001_101;
        board.o = 0b100_010_010;
        assert!(board.eval() > 900_000); // white should be crush
    }
}
