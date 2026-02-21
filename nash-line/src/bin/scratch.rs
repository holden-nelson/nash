use nash_line::editor::NashEditor;

fn main() {
    let mut ed: NashEditor = Default::default();

    ed.read_line().unwrap();
}
