use std::io;

use crate::core::EditorView;

pub trait Renderer {
    fn render(&mut self, editor_view: EditorView) -> io::Result<()>;
    fn commit(&mut self) -> io::Result<()>;
}
