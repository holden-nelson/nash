use std::io;

use crossterm::event::{Event, KeyCode, KeyModifiers, read};

use crate::core::EditorEvent;

pub trait EventSource {
    fn next_event(&mut self) -> io::Result<EditorEvent>;
}

pub struct TerminalEventSource;

impl EventSource for TerminalEventSource {
    fn next_event(&mut self) -> io::Result<EditorEvent> {
        loop {
            match read()? {
                Event::Key(event) => return match (event.code, event.modifiers) {
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) =>
                        Ok(EditorEvent::Char(c)),
                    (KeyCode::Backspace, _) => Ok(EditorEvent::Backspace),
                    (KeyCode::Delete, _) => Ok(EditorEvent::Delete),
                    (KeyCode::Enter, _) => Ok(EditorEvent::Enter),
                    (KeyCode::Tab, _) => Ok(EditorEvent::Tab),
                    (KeyCode::Left, _) => Ok(EditorEvent::Left),
                    (KeyCode::Right, _) => Ok(EditorEvent::Right),
                    (KeyCode::Home, _) => Ok(EditorEvent::Home),
                    (KeyCode::End, _) => Ok(EditorEvent::End),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => Ok(EditorEvent::CtrlC),
                    (KeyCode::Char('j'), KeyModifiers::CONTROL) => Ok(EditorEvent::CtrlJ),
                    _ => continue
                },
                _ => continue
            }
        }
    }
}