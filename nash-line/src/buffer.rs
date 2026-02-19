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

    pub fn print(&self) -> String {
        let mut output = String::new();

        for c in &self.left {
            output.push(*c);
        }

        for c in self.right.iter().rev() {
            output.push(*c);
        }

        output
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
        assert_eq!(buffer.print(), "a");
    }

    #[test]
    fn test_backspace() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.backspace();
        assert_eq!(buffer.print(), "");
    }

    #[test]
    fn test_delete_forward() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.move_cursor_right();
        buffer.delete_forward();
        assert_eq!(buffer.print(), "a");
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
}
