use crate::tui::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io;

// define the app
#[derive(Debug, Default)]
pub struct App {
    pub active_tab: MenuTabs,
    pub counter: u8,
    pub exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// render the frame
    fn render_frame(&self, frame: &mut Frame) {
        ui(frame, self);
    }

    /// update's application state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// handle key events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.active_tab = MenuTabs::Status,
            KeyCode::Char('2') => self.active_tab = MenuTabs::Log,
            KeyCode::Char('3') => self.active_tab = MenuTabs::Hangar,
            KeyCode::Char('4') => self.active_tab = MenuTabs::Crew,
            KeyCode::Char('5') => self.active_tab = MenuTabs::About,
            KeyCode::Char('6') => self.active_tab = MenuTabs::Help,
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    /// app methods
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
