use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::{
    core::{EditorCore, Step},
    events::{EventSource, TerminalEventSource},
    renderer::{Renderer, TerminalRenderer},
};

pub struct Editor<E: EventSource, R: Renderer> {
    core: EditorCore,
    events: E,
    renderer: R,
}

pub enum Signal {
    Aborted,
    Complete(String),
}

impl<E: EventSource, R: Renderer> Editor<E, R> {
    pub fn read_line(&mut self) -> io::Result<Signal> {
        self.core.reset();

        // ensures raw mode is disabled on exit
        let _raw = RawModeGuard::new()?;

        loop {
            self.renderer.render(self.core.view())?;

            let ev = self.events.next_event()?;
            match self.core.handle(ev) {
                Step::Continue => continue,
                Step::Completed => {
                    self.renderer.commit()?;
                    let line = self.core.take();
                    return Ok(Signal::Complete(line));
                }
                Step::Aborted => {
                    self.renderer.commit()?;
                    self.core.reset();
                    return Ok(Signal::Aborted);
                }
            }
        }
    }
}

pub type NashEditor = Editor<TerminalEventSource, TerminalRenderer<io::Stdout>>;

impl Default for Editor<TerminalEventSource, TerminalRenderer<io::Stdout>> {
    fn default() -> Self {
        Editor {
            core: EditorCore::new(),
            events: TerminalEventSource,
            renderer: TerminalRenderer::new(io::stdout()),
        }
    }
}

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
