use crate::buffer::Buffer;
use crossterm::{
    QueueableCommand, cursor,
    event::{Event, KeyCode, KeyModifiers, read},
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Stdout, Write};

pub struct Editor {
    buffer: Buffer,
}

#[derive(Debug)]
pub enum Signal {
    Completed(String),
    Aborted(String),
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buffer: Buffer::new(),
        }
    }

    pub fn read_line(&mut self) -> Signal {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();

        loop {
            Self::render(self, &stdout);

            match read().unwrap() {
                Event::Key(event) => match (event.code, event.modifiers) {
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                        self.buffer.insert(c);
                    }
                    (KeyCode::Backspace, _) => {
                        self.buffer.backspace();
                    }
                    (KeyCode::Delete, _) => {
                        self.buffer.delete_forward();
                    }
                    (KeyCode::Enter, _) => {
                        stdout
                            .queue(Print('\n'))
                            .unwrap()
                            .queue(cursor::MoveToColumn(0))
                            .unwrap()
                            .flush()
                            .unwrap();
                        self.buffer.insert('\n');
                        break;
                    }
                    (KeyCode::Tab, _) => {
                        todo!("Handle tab")
                    }

                    // navigation
                    (KeyCode::Left, _) => {
                        self.buffer.move_cursor_left();
                    }
                    (KeyCode::Right, _) => {
                        self.buffer.move_cursor_right();
                    }
                    (KeyCode::Home, _) => {
                        self.buffer.move_cursor_home();
                    }
                    (KeyCode::End, _) => {
                        self.buffer.move_cursor_end();
                    }

                    // control / signals
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        enable_raw_mode().unwrap();
                        return Signal::Aborted(self.buffer.print());
                    }
                    (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                        stdout
                            .queue(Print('\n'))
                            .unwrap()
                            .queue(cursor::MoveToColumn(0))
                            .unwrap()
                            .flush()
                            .unwrap();
                        self.buffer.insert('\n');
                        break;
                    }

                    // ignore
                    _ => {}
                },

                _ => {}
            }
        }

        disable_raw_mode().unwrap();

        Signal::Completed(self.buffer.print())
    }

    pub fn render(&self, mut stdout: &Stdout) {
        let final_cursor_position = self.buffer.cursor_column() + 2;

        stdout
            .queue(cursor::MoveToColumn(0))
            .unwrap()
            .queue(Clear(ClearType::CurrentLine))
            .unwrap()
            .queue(Print("$ "))
            .unwrap()
            .queue(Print(self.buffer.print()))
            .unwrap()
            .queue(cursor::MoveToColumn(
                final_cursor_position.try_into().unwrap(),
            ))
            .unwrap()
            .flush()
            .unwrap();
    }
}
