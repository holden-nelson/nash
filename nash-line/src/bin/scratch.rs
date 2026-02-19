use nash_line::editor::{Editor, Signal};

fn main() {
    let mut editor = Editor::new();
    match editor.read_line() {
        Signal::Completed(line) => println!("Completed: {:?}", line),
        Signal::Aborted(line) => println!("Aborted: {:?}", line),
    }
}
