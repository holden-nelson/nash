use std::io::{self, Write};

use crossterm::{
    QueueableCommand, cursor,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::core::EditorView;

pub trait Renderer {
    fn render(&mut self, editor_view: EditorView) -> io::Result<()>;
    fn commit(&mut self) -> io::Result<()>;
}

pub struct TerminalRenderer<W: Write> {
    out: W,
}

impl<W: Write> TerminalRenderer<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

impl<W: Write> Renderer for TerminalRenderer<W> {
    fn render(&mut self, editor_view: EditorView) -> io::Result<()> {
        let final_cursor_position = (editor_view.cursor_col + 2) as u16;

        self.out
            .queue(cursor::MoveToColumn(0))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print("$ "))?
            .queue(Print(editor_view.text))?
            .queue(cursor::MoveToColumn(final_cursor_position))?
            .flush()?;

        Ok(())
    }

    fn commit(&mut self) -> io::Result<()> {
        self.out
            .queue(Print('\n'))?
            .queue(cursor::MoveToColumn(0))?
            .flush()?;

        Ok(())
    }
}
