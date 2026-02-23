use std::time::{Duration, Instant};

use crate::{
    board::{Board, empty::toggle_empty, queen::toggle_queen, swapper::toggle_swap},
    event::{AppEvent, Event, EventHandler},
};
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub selected_pos: (u8, u8),
    pub events: EventHandler,
    pub original: Board,
    pub board: Board,
    pub auto_empty: bool,
    pub msg: String,
    pub did_win: bool,
    pub user_time: Instant,
    pub win_time: Option<Duration>,
}

impl App {
    pub fn new(board: String) -> Self {
        Self {
            running: true,
            selected_pos: (0, 0),
            events: EventHandler::new(),
            board: Board::load_board(&board),
            original: Board::load_board(&board),
            auto_empty: true,
            msg: String::new(),
            did_win: false,
            user_time: Instant::now(),
            win_time: None,
        }
    }
    pub fn tick(&self) {}
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn change_auto_empty(&mut self) {
        self.auto_empty = !self.auto_empty;
    }
    pub fn restart(&mut self) {
        let old_board = &self.original;
        let board = &mut self.board;
        board.queen_pos = old_board.queen_pos.clone();
        board.empty_pos = old_board.empty_pos.clone();
        self.msg = String::new();
        self.did_win = false;
        self.user_time = Instant::now();
        self.win_time = None;
        self.selected_pos = (0, 0);
    }

    /// true if the combination is correct
    /// else the string value has msg what is wrong
    pub fn check_win(&mut self) -> (bool, String) {
        let board = &mut self.board;
        let size = board.size.0 as usize;
        let queens = &board.queen_pos;

        if queens.len() != size {
            return (
                false,
                format!(
                    "Need exactly {} queens (currently have {})",
                    size,
                    queens.len()
                ),
            );
        }

        // 1. Check Rows and Columns
        let mut rows = vec![false; size];
        let mut cols = vec![false; size];

        for &(qx, qy) in queens {
            let (x, y) = (qx as usize, qy as usize);
            if rows[y] {
                return (false, format!("Row {} has more than one queen!", y + 1));
            }
            if cols[x] {
                return (false, format!("Column {} has more than one queen!", x + 1));
            }
            rows[y] = true;
            cols[x] = true;
        }

        for (i, region) in board.regions.iter().enumerate() {
            let mut queens_in_region = 0;
            for &pos in queens {
                if region.contains(&pos) {
                    queens_in_region += 1;
                }
            }
            if queens_in_region == 0 {
                return (false, format!("Region {} is missing a queen!", i));
            }
            if queens_in_region > 1 {
                return (false, format!("Region {} has multiple queens!", i));
            }
        }

        for i in 0..queens.len() {
            for j in i + 1..queens.len() {
                let (x1, y1) = (queens[i].0 as i16, queens[i].1 as i16);
                let (x2, y2) = (queens[j].0 as i16, queens[j].1 as i16);
                let dx = (x1 - x2).abs();
                let dy = (y1 - y2).abs();
                if dx <= 1 && dy <= 1 {
                    return (
                        false,
                        format!(
                            "Queens at ({},{}) and ({},{}) are touching!",
                            x1, y1, x2, y2
                        ),
                    );
                }
            }
        }

        (true, "You won!".to_string())
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::IncrementXPosition => self.change_position(1, 0),
                    AppEvent::DecrementXPosition => self.change_position(-1, 0),
                    AppEvent::IncrementYPosition => self.change_position(0, 1),
                    AppEvent::DecrementYPosition => self.change_position(0, -1),
                    AppEvent::Restart => self.restart(),
                    AppEvent::ChangeAutoEmpty => self.change_auto_empty(),
                    AppEvent::ToggleEmpty => {
                        toggle_empty(&mut self.board, self.selected_pos, self.auto_empty)
                    }
                    AppEvent::ToggleQueen => {
                        toggle_queen(&mut self.board, self.selected_pos, self.auto_empty);
                        let (did_win, msg) = self.check_win();
                        if did_win {
                            self.win_time = Some(self.user_time.elapsed());
                        }
                        self.did_win = did_win;
                        self.msg = msg;
                    }
                    AppEvent::Swap => {
                        toggle_swap(&mut self.board, self.selected_pos, self.auto_empty);
                        let (did_win, msg) = self.check_win();
                        if did_win {
                            self.win_time = Some(self.user_time.elapsed());
                        }
                        self.did_win = did_win;
                        self.msg = msg;
                    }
                },
            }
        }
        Ok(())
    }
    // Keyboard handle is in input.rs
}
