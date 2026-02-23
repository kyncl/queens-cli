use crate::{app::App, event::AppEvent};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::IncrementXPosition),
            KeyCode::Left => self.events.send(AppEvent::DecrementXPosition),
            KeyCode::Up => self.events.send(AppEvent::IncrementYPosition),
            KeyCode::Down => self.events.send(AppEvent::DecrementYPosition),
            KeyCode::Char('k') => self.events.send(AppEvent::ChangeAutoEmpty),
            KeyCode::Char('r') => self.events.send(AppEvent::Restart),
            KeyCode::Char('z') => self.events.send(AppEvent::ToggleQueen),
            KeyCode::Char('x') => self.events.send(AppEvent::ToggleEmpty),
            KeyCode::Char(' ') => self.events.send(AppEvent::Swap),
            _ => {}
        }
        Ok(())
    }
}
