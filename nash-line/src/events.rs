use std::io;

use crate::core::EditorEvent;

pub trait EventSource {
    fn next_event(&mut self) -> io::Result<EditorEvent>;
}
