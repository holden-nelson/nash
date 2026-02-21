//! Core logic for line editor
//!
//! Manages the input buffer and how incoming
//! editor events mutate it

use std::io;

use crate::buffer::{Buffer, BufferDisplay};
use EditorEvent::*;

pub enum EditorEvent {
    Char(char),
    Backspace,
    Delete,
    Enter,
    Tab,
    Left,
    Right,
    Home,
    End,
    CtrlC,
    CtrlJ,
}

pub struct EditorCore {
    buffer: Buffer,
}

#[derive(PartialEq, Debug)]
pub enum Step {
    Continue,
    Completed,
    Aborted,
}

impl EditorCore {
    pub fn new() -> Self {
        EditorCore {
            buffer: Buffer::new(),
        }
    }

    pub fn handle(&mut self, ev: EditorEvent) -> Step {
        match ev {
            Char(c) => self.buffer.insert(c),
            Backspace => self.buffer.backspace(),
            Delete => self.buffer.delete_forward(),
            Enter => {
                self.buffer.insert('\n');
                return Step::Completed;
            }
            Tab => {
                todo!()
            }
            Left => self.buffer.move_cursor_left(),
            Right => self.buffer.move_cursor_right(),
            Home => self.buffer.move_cursor_home(),
            End => self.buffer.move_cursor_end(),
            CtrlC => {
                return Step::Aborted;
            }
            CtrlJ => {
                self.buffer.insert('\n');
                return Step::Completed;
            }
        }

        Step::Continue
    }

    pub fn view(&self) -> EditorView<'_> {
        EditorView {
            text: self.buffer.as_display(),
            cursor_col: self.buffer.cursor_column(),
        }
    }

    pub fn take(&mut self) -> String {
        self.buffer.take_string()
    }

    pub fn reset(&mut self) {
        self.buffer.clear()
    }
}

pub struct EditorView<'a> {
    pub text: BufferDisplay<'a>,
    pub cursor_col: usize,
}

#[cfg(test)]
mod tests;
