use std::fmt::{self};

#[derive(Debug)]
pub struct Buffer {
    left: Vec<char>,
    right: Vec<char>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn insert(&mut self, c: char) {
        self.left.push(c);
    }

    pub fn backspace(&mut self) {
        self.left.pop();
    }

    pub fn delete_forward(&mut self) {
        self.right.pop();
    }

    pub fn replace(&mut self, s: &str) {
        self.left.clear();
        self.right.clear();
        for c in s.chars() {
            self.insert(c);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if let Some(c) = self.left.pop() {
            self.right.push(c);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(c) = self.right.pop() {
            self.left.push(c);
        }
    }

    pub fn move_cursor_home(&mut self) {
        while let Some(c) = self.left.pop() {
            self.right.push(c);
        }
    }

    pub fn move_cursor_end(&mut self) {
        while let Some(c) = self.right.pop() {
            self.left.push(c);
        }
    }

    pub fn cursor_column(&self) -> usize {
        self.left.len()
    }

    pub fn as_display(&self) -> BufferDisplay<'_> {
        BufferDisplay(self)
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
    }

    pub fn take_string(&mut self) -> String {
        let s = self
            .left
            .iter()
            .chain(self.right.iter().rev())
            .copied()
            .collect();

        self.clear();

        s
    }

    pub fn take_until_highlighted(&self) -> String {
        let right_until_whitespace = self.right.iter().rev().take_while(|&&c| !c.is_whitespace());

        self.left
            .iter()
            .chain(right_until_whitespace)
            .copied()
            .collect()
    }
}

pub struct BufferDisplay<'a>(&'a Buffer);

impl fmt::Display for BufferDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write as _;

        self.0
            .left
            .iter()
            .chain(self.0.right.iter().rev())
            .copied()
            .try_for_each(|c| f.write_char(c))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new();
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_insert() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        assert_eq!(buffer.as_display().to_string(), "a");
    }

    #[test]
    fn test_backspace() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.backspace();
        assert_eq!(buffer.as_display().to_string(), "");
    }

    #[test]
    fn test_delete_forward() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b'); // "ab"
        buffer.move_cursor_left(); // cursor between a|b (right contains 'b')
        buffer.delete_forward(); // delete 'b'
        assert_eq!(buffer.as_display().to_string(), "a");
        assert_eq!(buffer.cursor_column(), 1);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.move_cursor_left();
        assert_eq!(buffer.cursor_column(), 1);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.move_cursor_left();
        buffer.move_cursor_right();
        assert_eq!(buffer.cursor_column(), 2);
    }

    #[test]
    fn test_move_cursor_home() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.move_cursor_home();
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_move_cursor_end() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.move_cursor_end();
        assert_eq!(buffer.cursor_column(), 2);
    }

    #[test]
    fn test_clear_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.clear();
        assert_eq!(buffer.cursor_column(), 0);
        assert_eq!(buffer.as_display().to_string(), "");
    }

    #[test]
    fn test_clear_empties_buffer() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');

        assert_eq!(buffer.as_display().to_string(), "abc");

        buffer.clear();

        // buffer should be reset
        assert_eq!(buffer.cursor_column(), 0);
        assert_eq!(buffer.as_display().to_string(), "");
    }

    #[test]
    fn test_clear_with_cursor_in_middle_clears_both_sides() {
        let mut buffer = Buffer::new();
        buffer.replace("abcd"); // left = a b c d
        buffer.move_cursor_left(); // right = d
        buffer.move_cursor_left(); // right = d c, left = a b

        // Visible text should still be "abcd"
        assert_eq!(buffer.as_display().to_string(), "abcd");

        buffer.clear();

        // buffer should be reset
        assert_eq!(buffer.cursor_column(), 0);
        assert_eq!(buffer.as_display().to_string(), "");
    }

    #[test]
    fn test_clear_can_be_called_twice() {
        let mut buffer = Buffer::new();
        buffer.replace("hi");

        buffer.clear();
        buffer.clear();

        assert_eq!(buffer.cursor_column(), 0);
        assert_eq!(buffer.as_display().to_string(), "");
    }

    #[test]
    fn test_clear_after_edits() {
        let mut buffer = Buffer::new();
        buffer.replace("ab");
        buffer.insert('c'); // "abc"
        buffer.move_cursor_left(); // cursor between b and c
        buffer.backspace(); // removes 'b' => "ac"
        buffer.delete_forward(); // deletes 'c' (to the right) => "a"

        assert_eq!(buffer.as_display().to_string(), "a");

        buffer.clear();
        assert_eq!(buffer.as_display().to_string(), "");
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_take_string_on_empty_buffer() {
        let mut buffer = Buffer::new();
        assert_eq!(buffer.take_string(), "");
        assert_eq!(buffer.as_display().to_string(), "");
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_take_string_returns_contents_and_clears_buffer() {
        let mut buffer = Buffer::new();
        buffer.replace("hello");
        assert_eq!(buffer.take_string(), "hello");
        assert_eq!(buffer.as_display().to_string(), "");
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_take_string_with_cursor_in_middle_returns_full_visible_text() {
        let mut buffer = Buffer::new();
        buffer.replace("abcd");
        buffer.move_cursor_left();
        buffer.move_cursor_left(); // cursor between b|c
        assert_eq!(buffer.as_display().to_string(), "abcd");

        assert_eq!(buffer.take_string(), "abcd");
        assert_eq!(buffer.as_display().to_string(), "");
        assert_eq!(buffer.cursor_column(), 0);
    }

    #[test]
    fn test_take_string_can_be_called_twice() {
        let mut buffer = Buffer::new();
        buffer.replace("hi");

        assert_eq!(buffer.take_string(), "hi");
        assert_eq!(buffer.take_string(), "");
        assert_eq!(buffer.cursor_column(), 0);
    }
}
