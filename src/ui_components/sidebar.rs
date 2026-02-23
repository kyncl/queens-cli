use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::*,
};

pub fn render_sidebar(auto_empty: bool, sidebar_area: Rect, buf: &mut Buffer) {
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

    let auto_x_status = if auto_empty {
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
}
