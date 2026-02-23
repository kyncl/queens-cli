use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board = &self.loaded_board;
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

        let [col_labels_area, grid_rows_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner_area);
        let [row_labels_area, play_grid_area] =
            Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).areas(grid_rows_area);
        let [_col_spacer, col_numbers_area] =
            Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).areas(col_labels_area);
        let col_labels = Layout::horizontal(vec![Constraint::Length(7); board.size.0 as usize])
            .split(col_numbers_area);
        for (x, area) in col_labels.iter().enumerate() {
            Paragraph::new(x.to_string())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Indexed(245)))
                .render(*area, buf);
        }
        let row_label_items = Layout::vertical(vec![Constraint::Length(3); board.size.1 as usize])
            .split(row_labels_area);
        for (y, area) in row_label_items.iter().enumerate() {
            let center_y = area.y + 1;
            buf.set_string(
                area.x,
                center_y,
                y.to_string(),
                Style::default().fg(Color::Indexed(245)),
            );
        }

        let rows = Layout::vertical(vec![Constraint::Length(3); board.size.1 as usize])
            .split(play_grid_area);
        for (y, row_area) in rows.iter().enumerate() {
            let cols = Layout::horizontal(vec![Constraint::Length(7); board.size.0 as usize])
                .split(*row_area);
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

                let is_selected = current_pos == self.selected_pos;
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

        let sidebar_block = Block::bordered()
            .title(
                Line::from(" Status & Help ")
                    .centered()
                    .fg(Color::Yellow)
                    .bold(),
            )
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(240)))
            .padding(Padding::uniform(1));

        let auto_x_status = if self.auto_empty {
            Span::styled(" ON", Style::default().fg(Color::Green).bold())
        } else {
            Span::styled(" OFF", Style::default().fg(Color::Red).bold())
        };

        let help_text = vec![
            Line::from(vec![Span::raw("Auto-place X: "), auto_x_status]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" K      ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Toggle Auto-X"),
            ]),
            Line::from(vec![
                Span::styled(" X      ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Place X-Mark"),
            ]),
            Line::from(vec![
                Span::styled(" Z      ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Place Queen"),
            ]),
            Line::from(vec![
                Span::styled(" SPACE  ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Place X / Queen"),
            ]),
            Line::from(vec![
                Span::styled(
                    " SHIFT + Arrows ",
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw("Multiple Xs on board"),
            ]),
            Line::from(vec![
                Span::styled(" Arrows ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("Navigation"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" R      ", Style::default().fg(Color::Red).bold()),
                Span::raw("Restart"),
            ]),
            Line::from(vec![
                Span::styled(" Q      ", Style::default().fg(Color::Red).bold()),
                Span::raw("Quit Game"),
            ]),
        ];

        Paragraph::new(help_text)
            .block(sidebar_block)
            .render(sidebar_area, buf);

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
            let popup_area = Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(area)[1];

            let popup_area = Layout::horizontal([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(popup_area)[1];
            buf.set_style(popup_area, Style::default().bg(Color::Black));
            Clear.render(popup_area, buf);
            let popup_block = Block::bordered()
                .title(" VICTORY! ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow));

            Paragraph::new(vec![
                Line::from(self.msg.clone()).centered().bold(),
                Line::from(""),
                Line::from(format!(
                    "Time: {:.2?}",
                    if let Some(time) = self.win_time {
                        time
                    } else {
                        self.user_time.elapsed()
                    }
                ))
                .centered()
                .bold(),
                Line::from(""),
                Line::from("Press 'R' to play again").centered().italic(),
            ])
            .block(popup_block)
            .render(popup_area, buf);
        }
    }
}
