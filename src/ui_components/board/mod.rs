use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};

use crate::board::Board;

pub mod column_numbers;
pub mod row_numbers;

pub fn render_board(
    board: &Board,
    selected_pos: Option<(u8, u8)>,
    play_grid_area: Rect,
    buf: &mut Buffer,
) {
    let rows =
        Layout::vertical(vec![Constraint::Length(3); board.size.1 as usize]).split(play_grid_area);
    for (y, row_area) in rows.iter().enumerate() {
        let cols =
            Layout::horizontal(vec![Constraint::Length(7); board.size.0 as usize]).split(*row_area);
        for (x, cell_area) in cols.iter().enumerate() {
            let current_pos = (x as u8, y as u8);
            let region_color = board
                .regions
                .iter()
                .position(|r| r.contains(&current_pos))
                .map(|idx| match idx % 6 {
                    0 => Color::Red,
                    1 => Color::Blue,
                    2 => Color::Green,
                    3 => Color::Yellow,
                    4 => Color::Magenta,
                    _ => Color::Cyan,
                })
                .unwrap_or(Color::White);
            let (symbol, is_queen, is_mark) = if board.queen_pos.contains(&current_pos) {
                (board.queen_skin.clone(), true, false)
            } else if board.empty_pos.contains(&current_pos) {
                (board.empty_skin.clone(), false, true)
            } else {
                (String::from("·"), false, false)
            };

            let is_selected = if let Some(pos) = selected_pos
                && current_pos == pos
            {
                true
            } else {
                false
            };
            let cell_block = Block::default()
                .borders(Borders::ALL)
                .border_type(if is_selected {
                    BorderType::Double
                } else {
                    BorderType::Rounded
                })
                .border_style(Style::default().fg(if is_selected {
                    Color::White
                } else {
                    Color::Indexed(240)
                }))
                .style(Style::default().bg(region_color));
            let mut text_style = Style::default();
            text_style = text_style.fg(Color::Black);
            if is_queen {
                text_style = text_style.bold();
            } else if !is_mark {
                text_style = text_style.fg(Color::Indexed(245));
            }
            Paragraph::new(symbol)
                .alignment(Alignment::Center)
                .style(text_style)
                .block(cell_block)
                .render(*cell_area, buf);
        }
    }
}
