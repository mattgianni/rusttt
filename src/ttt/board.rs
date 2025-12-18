use crate::AppError;
use log::trace;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoardState {
    Open,
    Draw,
    Win(Player),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareState {
    Empty,
    Filled(Player),
}

impl Display for SquareState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Filled(player) => write!(f, "{player}"),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    squares: [SquareState; 9],
    move_number: u8,
}

impl Board {
    pub fn new() -> Board {
        trace!("Board::new() called.");
        Board {
            squares: [SquareState::Empty; 9],
            move_number: 0,
        }
    }

    // pub fn col_iter(&self, c: u8) -> impl Iterator<Item = &SquareState> {
    //     assert!(c < 3);
    //     let c = c as usize;
    //     self.squares[c * 3..(c * 3 + 3)].iter()
    // }

    // pub fn row_iter(&self, r: u8) -> impl Iterator<Item = &SquareState> {
    //     assert!(r < 3);
    //     let r = r as usize;
    //     self.squares[r..].iter().step_by(3)
    // }

    // pub fn fdiag_iter(&self) -> impl Iterator<Item = &SquareState> {
    //     self.squares[0..].iter().step_by(4)
    // }

    // pub fn bdiag_iter(&self) -> impl Iterator<Item = &SquareState> {
    //     self.squares[2..=6].iter().step_by(2)
    // }

    pub fn lines(&self) -> impl Iterator<Item = [&SquareState; 3]> {
        trace!("lines() called.");
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        LINES.into_iter().map(|idx| idx.map(|i| &self.squares[i]))
    }

    pub fn winner(&self) -> Option<Player> {
        trace!("winner() called.");

        self.lines()
            .filter_map(|line| {
                if line[0] == line[1] && line[1] == line[2] {
                    match *line[0] {
                        SquareState::Filled(player) => Some(player),
                        SquareState::Empty => None, // all the square empty
                    }
                } else {
                    None
                }
            })
            .next()
    }

    pub fn whose_turn(&self) -> Player {
        trace!("whose_turn called().");
        if self.move_number % 2 == 0 {
            Player::X
        } else {
            Player::O
        }
    }

    pub fn play_move(&mut self, square: u8) -> Result<(), AppError> {
        trace!("play_move({square}) called.");
        assert!(square >= 1 && square <= 9);
        let sqidx = (square - 1) as usize;

        if self.squares[sqidx] != SquareState::Empty {
            trace!(
                "returning error in play_move({square}) - attempt to move into a filled square!"
            );
            Err(AppError::Msg(
                "error: attempt to play a move on a filled square!".to_string(),
            ))
        } else {
            self.squares[sqidx] = SquareState::Filled(self.whose_turn());
            trace!("play_move incrementing move_number");
            self.move_number += 1;
            Ok(())
        }
    }

    pub fn unplay_move(&mut self, square: u8) -> Result<(), AppError> {
        trace!("unplay_move({square}) called.");
        assert!(square >= 1 && square <= 9);
        let sqidx = (square - 1) as usize;

        // can't unplay an empty square!
        if self.squares[sqidx] == SquareState::Empty {
            trace!("returning error in unplay_move({square}) - attempt to unmove an empty square!");
            return Err(AppError::Msg(
                "error: attempt to unmove an empty square!".to_string(),
            ));
        };

        // can't unplay someone else's move!
        if self.squares[sqidx] == SquareState::Filled(self.whose_turn()) {
            trace!("returning error in unplay_move({square}) - attempt to unmove out of turn!");
            return Err(AppError::Msg(
                "error: attempt to unmove out of turn!".to_string(),
            ));
        };

        // should be okay to unmove now
        self.squares[sqidx] = SquareState::Empty;
        trace!("play_move decrementing move_number");
        self.move_number -= 1;

        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        trace!("fmt(&Board, &formatter) called. (display)");
        for (i, sq) in self.squares.iter().enumerate() {
            match sq {
                SquareState::Empty => write!(f, "{}", i + 1)?,
                _ => write!(f, "{sq}")?,
            }
            if (i + 1) % 3 == 0 {
                writeln!(f)?
            } else {
                write!(f, " ")?
            }
        }
        Ok(())
    }
}
