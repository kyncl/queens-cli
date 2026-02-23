use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

pub fn render_column_nums(board_width: usize, col_labels_area: Rect, buf: &mut Buffer) {
    let [col_numbers_area] = Layout::horizontal([Constraint::Min(0)]).areas(col_labels_area);
    let col_labels =
        Layout::horizontal(vec![Constraint::Length(7); board_width]).split(col_numbers_area);
    for (x, area) in col_labels.iter().enumerate() {
        Paragraph::new((x + 1).to_string())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Indexed(245)))
            .render(*area, buf);
    }
}
