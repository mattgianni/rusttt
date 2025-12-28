#![allow(unused)]

use crate::{cli::Config, error::AppError, ttt::board::Board, ttt::game::Game};
use log::trace;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Gauge, Paragraph, Widget},
};

use rand::rng;
use rand::seq::IndexedRandom;

pub fn run(cfg: Config) -> Result<(), AppError> {
    trace!("run({:?}) called.", &cfg);
    // let _ = crate::ttt::game::play(&cfg)?;

    let mut app = App::new();
    let mut terminal = ratatui::init();
    let _app_result = app.run(&mut terminal);
    ratatui::restore();

    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
    pub game: Game,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            game: Game::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit(),
            KeyCode::Char('r') | KeyCode::Char('R') => self.game.board.play_random(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Board {
    fn play_random(&mut self) {
        let mut rng = rng();
        let moves: Vec<u8> = self.legal_moves().collect();
        if !moves.is_empty()
            && let Some(sq) = moves.choose(&mut rng)
        {
            self.play_move(*sq);
        } else {
            self.clear();
        }
    }

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

        Paragraph::new("game_stats_area")
            .centered()
            .block(block)
            .render(game_stats_area, buf);

        let instructions = Line::from(vec![
            " Random move ".into(),
            "<R>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from(" Bottom "))
            .title_bottom(instructions)
            .border_set(border::ROUNDED);

        Paragraph::new("bottom_area")
            .centered()
            .block(block)
            .render(bottom_area, buf);
    }
}
