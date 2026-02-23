use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};

use crate::app::App;

pub fn render_win_popup(app: &App, buf: &mut Buffer, area: Rect) {
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
        Line::from(app.msg.clone()).centered().bold(),
        Line::from(""),
        Line::from(format!(
            "Time: {:.2?}",
            if let Some(time) = app.win_time {
                time
            } else {
                app.user_time.elapsed()
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
