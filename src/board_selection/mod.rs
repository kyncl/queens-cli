use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
};

use crate::{
    board::{Board, BoardMetadata, Difficulties},
    event::{AppEvent, Event, EventHandler},
    ui_components::board::{
        column_numbers::render_column_nums, render_board, row_numbers::render_row_nums,
    },
};

pub struct BoardSelection {
    pub running: bool,
    pub events: EventHandler,
    pub boards: Vec<Board>,
    pub search_list_state: ListState,
}

impl Widget for &mut BoardSelection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board = self
            .boards
            .get(self.search_list_state.selected().unwrap_or(0));
        let board_size = if let Some(board) = board {
            board.size
        } else {
            (0, 0)
        };
        let board_width = (board_size.0 as u16) * 7 + 4;
        let header_height = 11u16;
        let info_height = 5u16;
        let [header_layout, main_layout, info_layout] = Layout::vertical([
            Constraint::Length(header_height),
            Constraint::Fill(1),
            Constraint::Length(info_height),
        ])
        .areas(area);
        let [header_area] = Layout::horizontal([Constraint::Fill(1)]).areas(header_layout);
        let [search_area, board_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(board_width)])
                .flex(Flex::Center)
                .areas(main_layout);
        let [info_area] = Layout::horizontal([Constraint::Fill(1)]).areas(info_layout);

        let header_text = vec![
            Line::from("                               /$$$$$$                                                            /$$$$$$  /$$       /$$$$$$").centered(),
            Line::from(r"                             /$$__  $$                                                          /$$__  $$| $$      |_  $$_/").centered(),
            Line::from(r"   \O/     \O/     \O/      | $$  \ $$ /$$   /$$  /$$$$$$   /$$$$$$  /$$$$$$$   /$$$$$$$       | $$  \__/| $$        | $$  ").centered(),
            Line::from(r"    |       |       |       | $$  | $$| $$  | $$ /$$__  $$ /$$__  $$| $$__  $$ /$$_____//$$$$$$| $$      | $$        | $$  ").centered(),
            Line::from(r"\  /|\     /|\     /|\  /   | $$  | $$| $$  | $$| $$$$$$$$| $$$$$$$$| $$  \ $$|  $$$$$$|______/| $$      | $$        | $$  ").centered(),
            Line::from(r" \/   \   / | \   /   \/    | $$/$$ $$| $$  | $$| $$_____/| $$_____/| $$  | $$ \____  $$       | $$    $$| $$        | $$  ").centered(),
            Line::from(r"  \    \_/  |  \_/    /     |  $$$$$$/|  $$$$$$/|  $$$$$$$|  $$$$$$$| $$  | $$ /$$$$$$$/       |  $$$$$$/| $$$$$$$$ /$$$$$$").centered(),
            Line::from(r"   `-------[♛]-------'       \____ $$$ \______/  \_______/ \_______/|__/  |__/|_______/         \______/ |________/|______/").centered(),
            Line::from(r"                                   \__/                                                                                     ").centered(),
        ];
        Paragraph::new(header_text)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(header_area, buf);

        let outer_block = Block::new()
            .borders(Borders::all())
            .title("Board")
            .border_type(BorderType::Rounded);
        if let Some(board) = board {
            let inner_area = outer_block.inner(board_area);
            let [col_labels_area, grid_rows_area] =
                Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner_area);
            // needs space for row numbers
            let [_, play_grid_area] =
                Layout::horizontal([Constraint::Length(2), Constraint::Min(0)])
                    .areas(grid_rows_area);
            render_column_nums(board.size.0 as usize, col_labels_area, buf);
            render_row_nums(board.size.1 as usize, grid_rows_area, buf);
            render_board(board, None, play_grid_area, buf);
            outer_block.render(board_area, buf);
        }

        let search_block = Block::new()
            .borders(Borders::all())
            .title("Search")
            .border_type(BorderType::Rounded);
        let search_inner = search_block.inner(search_area);
        search_block.render(search_area, buf);
        let board_items: Vec<ListItem> = self
            .boards
            .iter()
            .map(|b| {
                let label = b
                    .metadata
                    .as_ref()
                    .map(|m| {
                        let best_line = match m.users_best {
                            Some(duration) => {
                                let total_secs = duration.as_secs();
                                let mins = total_secs / 60;
                                let secs = total_secs % 60;
                                let millis = duration.subsec_millis();
                                if mins > 0 {
                                    format!("Your best: {}m {:02}s", mins, secs)
                                } else {
                                    format!("Your best: {}.{:03}s", secs, millis)
                                }
                            }
                            None => "Your best: Not record".to_string(),
                        };
                        format!("{} - {} ({})", m.author, m.difficulty, best_line)
                    })
                    .unwrap_or_else(|| format!("{}x{}", b.size.0, b.size.1));
                ListItem::new(label)
            })
            .collect();
        let board_list = List::new(board_items)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(ratatui::style::Color::Cyan),
            );
        StatefulWidget::render(board_list, search_inner, buf, &mut self.search_list_state);

        if let Some(board) = &self
            .boards
            .get(self.search_list_state.selected().unwrap_or(0))
            && let Some(metadata) = &board.metadata
        {
            let author_line = match &metadata.author_link {
                Some(link) => format!(" Author : {} ({})", metadata.author, link),
                None => format!(" Author : {}", metadata.author),
            };

            let best_line = match metadata.users_best {
                Some(duration) => {
                    let total_secs = duration.as_secs();
                    let mins = total_secs / 60;
                    let secs = total_secs % 60;
                    let millis = duration.subsec_millis();
                    if mins > 0 {
                        format!(" Your best : {}m {:02}s", mins, secs)
                    } else {
                        format!(" Your best : {}.{:03}s", secs, millis)
                    }
                }
                None => " Your best : —".to_string(),
            };

            let info_text = vec![
                Line::from(author_line),
                Line::from(format!(" Difficulty: {}", metadata.difficulty)),
                Line::from(best_line),
            ];

            let info_block = Block::new()
                .borders(Borders::ALL)
                .title(" Info ")
                .border_type(BorderType::Rounded);

            Paragraph::new(info_text)
                .block(info_block)
                .render(info_area, buf);
        }
    }
}

impl BoardSelection {
    pub fn new() -> Self {
        let boards = vec![
            // ── 6x6 EASY ──────────────────────────────────────────────────────────────
            // Region layout:
            // 0 0 2 2 3 3
            // 0 1 1 2 3 3
            // 0 1 1 2 3 4
            // 0 1 5 5 4 4
            // 0 5 5 4 4 4
            Board::load_board(
                "6x6|0:0,0;1,0;2,0;2,1;1:0,1;0,2;0,3;1,3;2:3,0;1,1;2,2;3,1;1,2;3,2;3:4,0;5,0;4,1;4,2;2,3;3,3;4,3;4:5,1;5,2;5,3;5,4;5:0,4;1,4;2,4;3,4;4,4;0,5;1,5;2,5;3,5;4,5;5,5;|Q:|X:",
                Some(BoardMetadata::new(
                    "Gregory House",
                    Some("https://github.com/kyncl"),
                    Difficulties::Easy,
                    None,
                )),
            ),
            // Region layout:
            // 0 0 0 1 1 1
            // 0 0 1 1 2 2
            // 3 0 1 2 2 2
            // 3 3 4 2 2 5
            // 3 3 4 4 5 5
            // 3 4 4 5 5 5
            Board::load_board(
                "6x6|0:0,0;1,0;0,1;1,1;0,2;1,2;1:2,0;3,0;2,1;3,1;2,2;3,2;2,3;3,3;2:4,0;5,0;4,1;5,1;4,2;5,2;3:0,3;1,3;0,4;1,4;0,5;1,5;4:4,3;5,3;5,4;4,5;5,5;5:2,4;3,4;4,4;2,5;3,5;|Q:|X:",
                Some(BoardMetadata::new(
                    "James Wilson",
                    None,
                    Difficulties::Easy,
                    None,
                )),
            ),
            // Region layout:
            // 0 0 0 1 1 1 1
            // 0 0 1 1 2 2 1
            // 0 3 3 2 2 2 1
            // 3 3 3 2 4 4 6
            // 3 5 5 4 4 6 6
            // 5 5 5 4 6 6 6
            // 5 5 4 4 6 6 6  -- 7 regions for 7x7
            Board::load_board(
                "7x7|0:0,0;1,0;2,0;0,1;1,1;0,2;1:3,0;4,0;5,0;6,0;2,1;3,1;6,1;6,2;2:4,1;5,1;3,2;4,2;5,2;3,3;3:1,2;2,2;0,3;1,3;2,3;0,4;4:4,3;5,3;3,4;4,4;2,5;3,5;2,6;3,6;5:1,4;2,4;0,5;1,5;2,5;0,6;1,6;6:6,3;5,4;6,4;4,5;5,5;6,5;4,6;5,6;6,6;|Q:|X:",
                Some(BoardMetadata::new(
                    "Lisa Cuddy",
                    None,
                    Difficulties::Normal,
                    None,
                )),
            ),
            // ── 8x8 MEDIUM ────────────────────────────────────────────────────────────
            // 8 regions for 8x8
            // Region layout:
            // 0 0 0 0 1 1 1 1
            // 0 0 2 2 2 1 1 1
            // 0 2 2 2 3 3 1 1
            // 2 2 4 3 3 3 1 5
            // 4 4 4 3 3 6 5 5
            // 4 4 7 7 6 6 5 5
            // 4 7 7 7 6 6 5 5  -- wrong col count, be careful
            // 7 7 7 6 6 5 5 5  -- this row is fine
            Board::load_board(
                "8x8|0:0,0;1,0;2,0;3,0;0,1;1,1;0,2;1:4,0;5,0;6,0;7,0;5,1;6,1;7,1;6,2;7,2;6,3;2:2,1;3,1;2,2;3,2;4,2;1,2;0,3;1,3;2,3;3:4,3;5,3;3,3;4,4;5,4;3,4;4,1;4,5;3,5;4:2,4;1,4;0,4;2,5;1,5;0,5;2,6;1,6;5:7,3;7,4;6,4;7,5;6,5;7,6;6,6;5,6;6,7;5,7;7,7;6:5,5;4,6;5,6;3,6;4,7;3,7;7:0,6;1,7;0,7;2,7;|Q:|X:",
                Some(BoardMetadata::new(
                    "Eric Foreman",
                    None,
                    Difficulties::Normal,
                    None,
                )),
            ),
            // ── 8x8 HARD ──────────────────────────────────────────────────────────────
            // Irregular snake-like regions, harder to solve
            // 0 0 1 1 1 2 2 2
            // 0 1 1 3 2 2 4 2
            // 0 1 3 3 3 4 4 2
            // 0 3 3 5 4 4 6 6
            // 7 3 5 5 5 6 6 6
            // 7 7 5 5 6 6 4 6  -- fix: need 8 regions
            // 7 7 7 5 6 4 4 6
            // 7 7 6 6 6 4 4 4  -- 8 regions: 0-7
            Board::load_board(
                "8x8|0:0,0;1,0;0,1;0,2;0,3;1:2,0;3,0;4,0;1,1;2,1;1,2;1,3;2:5,0;6,0;7,0;4,1;7,1;7,2;5:3,2;4,2;5,2;2,3;3,3;2,4;3,4;3:3,1;2,2;4,3;5,3;3,5;4,5;3,6;4,6;4:6,1;5,1;5,4;6,4;7,3;4,4;5,6;5,7;6:6,3;7,4;6,5;7,5;5,5;6,6;4,7;5,6;7:0,4;1,4;0,5;1,5;0,6;1,6;2,6;0,7;1,7;2,7;|Q:|X:",
                Some(BoardMetadata::new(
                    "Allison Cameron",
                    None,
                    Difficulties::Hard,
                    None,
                )),
            ),
            // ── 9x9 HARD ──────────────────────────────────────────────────────────────
            // 9 regions for 9x9 — large spiral-like layout
            Board::load_board(
                "9x9|0:0,0;1,0;2,0;3,0;0,1;0,2;0,3;0,4;1:4,0;5,0;6,0;7,0;8,0;8,1;8,2;8,3;2:1,1;2,1;3,1;4,1;5,1;6,1;7,1;7,2;7,3;3:1,2;2,2;3,2;4,2;5,2;6,2;6,3;6,4;4:1,3;2,3;3,3;4,3;5,3;5,4;5,5;5,6;5:1,4;2,4;3,4;4,4;4,5;4,6;4,7;6:8,4;7,4;6,4;6,5;6,6;6,7;6,8;7,8;7:0,5;0,6;0,7;0,8;1,8;2,8;3,8;4,8;8:1,5;1,6;2,5;2,6;2,7;3,5;3,6;3,7;9:1,7;7,5;7,6;7,7;8,5;8,6;8,7;8,8;5,7;5,8;|Q:|X:",
                Some(BoardMetadata::new(
                    "Lawrence Kutner",
                    None,
                    Difficulties::Hard,
                    None,
                )),
            ),
        ];
        let mut list_state = ListState::default();
        if !boards.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            running: true,
            events: EventHandler::new(),
            boards: boards,
            search_list_state: list_state,
        }
    }
    pub fn next_board(&mut self) {
        let i = match self.search_list_state.selected() {
            Some(i) => {
                if i >= self.boards.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.search_list_state.select(Some(i));
    }
    pub fn prev_board(&mut self) {
        let i = match self.search_list_state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.boards.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.search_list_state.select(Some(i));
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn tick(&self) {}
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<Option<Board>> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
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
                    AppEvent::IncrementYPosition => self.prev_board(),
                    AppEvent::DecrementYPosition => self.next_board(),
                    AppEvent::Confirm => {
                        self.quit();
                        if let Some(selected_board) = self
                            .boards
                            .get(self.search_list_state.selected().unwrap_or(0))
                        {
                            return Ok(Some(selected_board.clone()));
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(None)
    }
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Ok(self.events.send(AppEvent::Quit)),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                Ok(self.events.send(AppEvent::Quit))
            }
            KeyCode::Up => Ok(self.events.send(AppEvent::IncrementYPosition)),
            KeyCode::Down => Ok(self.events.send(AppEvent::DecrementYPosition)),
            KeyCode::Enter => Ok(self.events.send(AppEvent::Confirm)),
            _ => Ok(()),
        }
    }
}
