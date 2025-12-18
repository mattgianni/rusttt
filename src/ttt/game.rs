// struct Game;
use crate::cli::Config;
use crate::error::AppError;
use crate::ttt::board::Board;
use log::trace;

pub fn play(cfg: &Config) -> Result<String, AppError> {
    trace!("play({:?}) called.", cfg);

    let mut board = Board::new();

    board.play_move(6)?;
    board.play_move(5)?;
    board.play_move(3)?;
    board.play_move(1)?;

    print!("{board}");
    println!("Player {}'s turn to move.", board.whose_turn());

    println!("Current winner: {:?}", board.winner());
    board.play_move(9)?;

    print!("{board}");
    println!("Current winner: {:?}", board.winner());

    // unplay last move
    board.unplay_move(9)?;
    print!("{board}");
    println!("Current winner: {:?}", board.winner());

    Ok(format!("processed: {:?}", cfg))
}
