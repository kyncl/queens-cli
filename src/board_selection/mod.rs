use crossterm::event::KeyEvent;
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect, widgets::Widget};

use crate::event::{AppEvent, Event, EventHandler};

pub struct BoardSelection {
    pub running: bool,
    pub events: EventHandler,
}

impl Widget for &BoardSelection {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}

impl BoardSelection {
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn tick(self) {}
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
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
                    AppEvent::IncrementYPosition => {}
                    AppEvent::DecrementYPosition => {}
                    _ => {}
                },
            }
        }
        Ok(())
    }
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            _ => Ok(()),
        }
    }
}
