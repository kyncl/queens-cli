use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
};

pub fn render_row_nums(board_height: usize, grid_rows_area: Rect, buf: &mut Buffer) {
    let [row_labels_area] = Layout::horizontal([Constraint::Length(2)]).areas(grid_rows_area);
    let row_label_items =
        Layout::vertical(vec![Constraint::Length(3); board_height]).split(row_labels_area);
    for (y, area) in row_label_items.iter().enumerate() {
        let center_y = area.y + 1;
        buf.set_string(
            area.x,
            center_y,
            (y + 1).to_string(),
            Style::default().fg(Color::Indexed(245)),
        );
    }
}
