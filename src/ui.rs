use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};

use crate::{
    app::App,
    ui_components::{
        board::{column_numbers::render_column_nums, render_board, row_numbers::render_row_nums},
        sidebar::render_sidebar,
        win_popup::render_win_popup,
    },
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board = &self.board;
        let board_width = (board.size.0 as u16) * 7 + 4;
        let board_height = (board.size.1 as u16) * 3 + 3;
        let info_bar_height = 15;
        let sidebar_width = 28;
        let [main_layout, info_layout] = Layout::vertical([
            Constraint::Length(board_height),
            Constraint::Length(info_bar_height),
        ])
        .flex(Flex::Center)
        .areas(area);

        let [board_area, sidebar_area] = Layout::horizontal([
            Constraint::Length(board_width),
            Constraint::Length(sidebar_width),
        ])
        .flex(Flex::Center)
        .areas(main_layout);

        let outer_block = Block::bordered()
            .title(
                Line::from(" Queens Game ")
                    .centered()
                    .fg(Color::Yellow)
                    .bold(),
            )
            .title_bottom(Line::from(format!(
                "Time: {:.2?}",
                if let Some(time) = self.win_time {
                    time
                } else {
                    self.user_time.elapsed()
                }
            )))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(240)));

        outer_block.clone().render(board_area, buf);
        let inner_area = outer_block.inner(board_area);

        // Numbering
        let [col_labels_area, grid_rows_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner_area);
        // needs space for row numbers
        let [_, play_grid_area] =
            Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).areas(grid_rows_area);
        render_column_nums(board.size.0 as usize, col_labels_area, buf);
        render_row_nums(board.size.1 as usize, grid_rows_area, buf);

        render_board(board, Some(self.selected_pos), play_grid_area, buf);
        render_sidebar(self.auto_empty, sidebar_area, buf);

        // Game status
        let msg_style = if self.did_win {
            Style::default().fg(Color::Green).bold()
        } else {
            Style::default().fg(Color::LightRed)
        };
        Paragraph::new(self.msg.clone())
            .alignment(Alignment::Center)
            .style(msg_style)
            .render(info_layout, buf);
        if self.did_win {
            render_win_popup(self, buf, area);
        }
    }
}
