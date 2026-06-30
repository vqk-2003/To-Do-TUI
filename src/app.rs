use crate::ui::render;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use bool as IsFinished;

pub struct App {
    pub(crate) screen: CurrentScreen,
    pub(crate) cursor_line: usize,
    pub(crate) list: Vec<(IsFinished, String)>,
}

pub(crate) enum CurrentScreen {
    Main,
    Editing,
    Deleting,
    Exiting,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: CurrentScreen::Main,
            cursor_line: 0,
            list: vec![
                (false, "Hello, World".into()),
                (false, "Good morning".into()),
            ],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| render(frame, self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                if KeyCode::Char('q') == key.code {
                    break Ok(());
                }

                match self.screen {
                    CurrentScreen::Main => self.handle_main_key_event(key),
                    CurrentScreen::Editing => self.handle_editing_key_event(key),
                    CurrentScreen::Deleting => match key.code {
                        KeyCode::Char('y') => {
                            self.list.remove(self.cursor_line);
                            self.screen = CurrentScreen::Main;
                        }
                        KeyCode::Char('n') => {
                            self.screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            self.save_list();
                            break Ok(());
                        }
                        KeyCode::Char('n') => break Ok(()),
                        KeyCode::Char('c') => self.screen = CurrentScreen::Main,
                        _ => {}
                    },
                }
            }
        }
    }

    fn handle_main_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let cursor_line = self.cursor_line.wrapping_add(1);
                self.cursor_line = if cursor_line >= self.list.len() {
                    0
                } else {
                    cursor_line
                };
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let cursor_line = self.cursor_line.wrapping_sub(1);
                self.cursor_line = if self.list.len() == 0 {
                    0
                } else if cursor_line >= self.list.len() {
                    self.list.len() - 1
                } else {
                    cursor_line
                };
            }
            KeyCode::Char('t') => {
                self.list[self.cursor_line].0 = !self.list[self.cursor_line].0;
            }
            KeyCode::Char('a') => {
                self.cursor_line = self.list.len();
                self.list.push((false, String::new()));
                self.screen = CurrentScreen::Editing;
            }
            KeyCode::Char('e') => {
                if self.list.len() != 0 {
                    self.screen = CurrentScreen::Editing;
                }
            }
            KeyCode::Char('x') => {
                self.screen = CurrentScreen::Deleting;
            }
            KeyCode::Char('q') => {
                self.screen = CurrentScreen::Exiting;
            }
            _ => {}
        }
    }

    fn handle_editing_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(value) => self.list[self.cursor_line].1.push(value),
            KeyCode::Backspace => {
                self.list[self.cursor_line].1.pop();
            }
            KeyCode::Enter => self.screen = CurrentScreen::Main,
            _ => {}
        }
    }

    fn save_list(&mut self) {}
}
