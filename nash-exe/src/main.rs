use nash_core::runner;
use nash_line::editor::{NashEditor, Signal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ed: NashEditor = Default::default();

    loop {
        let line = ed.read_line()?;

        match line {
            Signal::Complete(l) => runner::run(&l)?,
            Signal::Aborted => break,
        }
    }

    Ok(())
}
