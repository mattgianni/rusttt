use crate::{
    cli::Config,
    error::AppError,
    ttt::{board::Board, engine::negamax_ab, game::Game, player::Player},
};
use log::{debug, trace};

use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use rand::Rng;

pub fn run(cfg: Config) -> Result<(), AppError> {
    trace!("run({:?}) called.", &cfg);

    let mut app = App::new();
    let mut terminal = ratatui::init();
    let _app_result = app.run(&mut terminal);
    ratatui::restore();

    Ok(())
}

fn max_by_key_random<'a, T, K: Ord>(
    iter: impl Iterator<Item = &'a T>,
    mut key_fn: impl FnMut(&T) -> K,
) -> Option<&'a T> {
    let mut rng = rand::rng();

    let mut best: Option<&T> = None;
    let mut best_key: Option<K> = None;
    let mut ties = 0;

    for item in iter {
        let key = key_fn(item);

        match &best_key {
            None => {
                best = Some(item);
                best_key = Some(key);
                ties = 1;
            }
            Some(bk) if key > *bk => {
                best = Some(item);
                best_key = Some(key);
                ties = 1;
            }
            Some(bk) if key == *bk => {
                ties += 1;
                if rng.random_range(0..ties) == 0 {
                    best = Some(item);
                }
            }
            _ => {}
        }
    }

    best
}

pub enum AppMessage {
    KeyEvent(crossterm::event::KeyEvent),
    Random(u8),
}

type GameHistory = Vec<(String, Vec<u8>)>;
#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
    pub game: Game,
    pub games_played: u128,
    pub x_wins: u128,
    pub o_wins: u128,
    pub game_history: GameHistory,
    pub autoplay_enabled: Arc<AtomicBool>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            game: Game::new(),
            games_played: 0,
            x_wins: 0,
            o_wins: 0,
            game_history: Vec::<(String, Vec<u8>)>::new(),
            autoplay_enabled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (tx, rx) = mpsc::channel::<AppMessage>();

        let input_tx = tx.clone();
        thread::spawn(move || {
            debug!("spawning event handler.");
            Self::background_event_handler(input_tx);
        });

        let rand_tx = tx.clone();
        let autoplay_enabled = Arc::clone(&self.autoplay_enabled);
        thread::spawn(move || {
            debug!("spawning background rand gen.");
            Self::background_rand(rand_tx, autoplay_enabled);
        });

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(&rx)?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn background_rand(tx: mpsc::Sender<AppMessage>, autoplay_enabled: Arc<AtomicBool>) {
        let mut rng = rand::rng();
        loop {
            let r = rng.random_range(0..9) as u8;
            if autoplay_enabled.load(Ordering::Relaxed) {
                if tx.send(AppMessage::Random(r)).is_err() {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn background_event_handler(tx: mpsc::Sender<AppMessage>) {
        loop {
            match crossterm::event::read() {
                Ok(crossterm::event::Event::Key(key_event)) => {
                    if tx.send(AppMessage::KeyEvent(key_event)).is_err() {
                        break;
                    }
                }
                Ok(_) => {}
                Err(err) => {
                    eprintln!("event::read() failed: {err}");
                    break;
                }
            }
        }
    }

    fn handle_events(&mut self, rx: &mpsc::Receiver<AppMessage>) -> io::Result<()> {
        match rx.recv() {
            Ok(AppMessage::KeyEvent(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Ok(AppMessage::Random(n)) => self.handle_random(n),
            _ => {}
        };
        Ok(())
    }

    fn handle_random(&mut self, n: u8) {
        let _n = n;
        if self.game.board.legal_moves_safe().next() == None {
            self.reset_game();
        } else {
            self.play_best();
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // handle quit right away
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit(),
            _ => {}
        }

        // reset the board if no legal moves exist, otherwise handle keys normally
        if self.game.board.legal_moves_safe().next() == None {
            self.reset_game();
        } else {
            match key_event.code {
                KeyCode::Char('r') | KeyCode::Char('R') => self.game.play_random(),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let sq = c.to_digit(10).unwrap() as u8;

                    if sq >= 1 && self.game.board.get(sq - 1) == None {
                        self.game.play_move(sq - 1);
                    };
                }
                KeyCode::Char('b') | KeyCode::Char('B') => self.play_best(),
                KeyCode::Char('c') | KeyCode::Char('C') => self.reset_game(),
                KeyCode::Char('f') | KeyCode::Char('F') => self.full_reset(),
                KeyCode::Char('a') | KeyCode::Char('A') => self.toggle_auto_play(),
                _ => {}
            }
        }
    }

    fn toggle_auto_play(&mut self) {
        let new_value = !self.autoplay_enabled.load(Ordering::Relaxed);
        self.autoplay_enabled.store(new_value, Ordering::Relaxed);
    }

    fn reset_game(&mut self) {
        if self.game.board.legal_moves_safe().next() == None {
            if self.game.board.winner() == Some(Player::X) {
                self.x_wins += 1;
                self.game_history
                    .push((String::from("X Wins"), self.game.moves.clone()));
            } else if self.game.board.winner() == Some(Player::O) {
                self.o_wins += 1;
                self.game_history
                    .push((String::from("O Wins"), self.game.moves.clone()));
            } else {
                self.game_history
                    .push((String::from("Draw"), self.game.moves.clone()));
            }
            self.games_played += 1;
        } else {
            self.game_history
                .push((String::from("aborted"), self.game.moves.clone()));
        }
        self.game.clear();
    }

    fn full_reset(&mut self) {
        self.reset_game();
        self.o_wins = 0;
        self.x_wins = 0;
        self.games_played = 0;
        self.game_history.clear();
    }

    fn play_best(&mut self) {
        if let Some((sq, _score)) = self.best_move() {
            self.game.play_move(sq);
        }
    }

    fn best_move(&self) -> Option<(u8, i32)> {
        if self.game.board.legal_moves_safe().next() == None {
            return None;
        }

        let moves = self.evaluate_moves();
        let (sq, score) = max_by_key_random(moves.iter(), |(_sq, score)| *score)
            .copied()
            .unwrap();

        Some((sq, score))
    }

    fn evaluate_moves(&self) -> Vec<(u8, i32)> {
        let (alpha, beta) = (i32::MIN + 1000, i32::MAX - 1000);
        let mut board = self.game.board.clone();
        let mut moves: Vec<(u8, i32)> = Vec::new();

        for sq in board.legal_moves() {
            board.play_move(sq);

            let score = -negamax_ab(&board, 9, -beta, -alpha);
            // alpha = alpha.max(score);
            moves.push((sq, score));

            board.unplay_move(sq);
        }

        moves
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Board {
    fn row_line(self, n: u8) -> String {
        assert!(n < 3);
        let squares: Vec<String> = (0..9)
            .map(|sq| {
                if let Some(player) = self.get(sq) {
                    player.to_string()
                } else {
                    format!("{}", sq + 1)
                }
            })
            .collect();

        let start: usize = (n * 3) as usize;
        format!(
            " {} | {} | {} ",
            squares[start],
            squares[start + 1],
            squares[start + 2]
        )
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical_layout = Layout::vertical([Constraint::Length(7), Constraint::Fill(1)]);
        let horizontal_layout = Layout::horizontal([Constraint::Length(17), Constraint::Fill(1)]);

        let [top_area, bottom_area] = vertical_layout.areas(area);
        let [game_area, game_stats_area] = horizontal_layout.areas(top_area);

        let block = Block::bordered()
            .title(Line::from(" Game "))
            .border_set(border::ROUNDED);

        let board = self.game.board;
        let game_board = vec![
            Line::from(board.row_line(0_u8)),
            Line::from("---+---+---"),
            Line::from(board.row_line(1_u8)),
            Line::from("---+---+---"),
            Line::from(board.row_line(2_u8)),
        ];

        Paragraph::new(game_board)
            .centered()
            .block(block)
            .render(game_area, buf);

        let block = Block::bordered()
            .title(Line::from(" Stats "))
            .border_set(border::ROUNDED);

        let legal: Vec<u8> = board.legal_moves_safe().collect();
        let winner = match board.winner() {
            Some(player) => player.to_string(),
            None => "None".to_string(),
        };

        let turn_line = Line::from(vec![
            format!(" Turn: ").into(),
            format!("{} to move", board.turn).bold(),
        ]);

        let legal_line = Line::from(vec![
            format!(" Legal: ").into(),
            format!("{:?}", legal).bold(),
        ]);

        let winner_line = Line::from(vec![
            format!(" Winner: ").into(),
            format!("{}", winner).bold(),
        ]);

        let best_line = Line::from(vec![
            format!(" Best: ").into(),
            format!("{:?}", self.best_move()).bold(),
        ]);

        let start_time = Instant::now();
        let move_eval = self.evaluate_moves();
        let elapsed = start_time.elapsed();
        let elapsed_str = match elapsed.as_nanos() {
            nano if nano > 1_200_000_000 => format!("{} s", elapsed.as_secs()),
            nano if nano > 1_200_000 => format!("{} ms", elapsed.as_millis()),
            nano if nano > 2_500 => format!("{} \u{00B5}s", elapsed.as_micros()),
            nano => format!("{} ns", nano),
        };

        let eval_line = Line::from(vec![
            format!(" Eval[{}]: ", elapsed_str).into(),
            format!("{:?}", move_eval).bold(),
        ]);

        Paragraph::new(vec![
            turn_line,
            legal_line,
            winner_line,
            best_line,
            eval_line,
        ])
        .block(block)
        .render(game_stats_area, buf);

        let instructions = Line::from(vec![
            " Make move ".into(),
            "<[1-9]>".blue().bold(),
            " Best move ".into(),
            "<B>".blue().bold(),
            " Random move ".into(),
            "<R>".blue().bold(),
            " Auto move ".into(),
            "<A>".blue().bold(),
            " Clear ".into(),
            "<C> ".blue().bold(),
            " Full reset ".into(),
            "<F> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from(" History "))
            .title_bottom(instructions)
            .border_set(border::ROUNDED);

        let wins_line = Line::from(vec![
            format!(" Wins - X: ").into(),
            format!("{}", self.x_wins).bold(),
            format!(" O: ").into(),
            format!("{}", self.o_wins).bold(),
        ]);

        let games_played_line = Line::from(vec![
            format!(" Games played: ").into(),
            format!("{}", self.games_played).bold(),
        ]);

        let mut game_lines: Vec<Line<'_>> = self
            .game_history
            .iter()
            .rev()
            .map(|g| Line::from(format!(" {} - {:?}", g.0, g.1)))
            .collect();

        let mut bottom_text = vec![wins_line, games_played_line];
        bottom_text.append(&mut game_lines);

        Paragraph::new(bottom_text)
            .block(block)
            .render(bottom_area, buf);
    }
}
