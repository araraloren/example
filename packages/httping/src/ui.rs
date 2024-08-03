use std::io::Result;
use std::io::Write;
use std::time::Duration;

use ratatui::crossterm::event::poll;
use ratatui::crossterm::event::read;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::disable_raw_mode;
use ratatui::crossterm::terminal::enable_raw_mode;
use ratatui::crossterm::terminal::EnterAlternateScreen;
use ratatui::crossterm::terminal::LeaveAlternateScreen;
use ratatui::prelude::*;

#[derive(Debug)]
pub struct Ui<B: Write> {
    terminal: Terminal<CrosstermBackend<B>>,
}

impl<B: Write> Ui<B> {
    pub fn new(mut backend: B) -> Result<Self> {
        enable_raw_mode()?;
        execute!(backend, EnterAlternateScreen)?;
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(backend))?,
        })
    }

    pub fn run_loop<A, U, V>(&mut self, a: &mut A, v: V, u: U) -> Result<()>
    where
        V: FnMut(&mut A, &mut Frame),
        U: FnMut(&mut A, Event) -> Result<bool>,
    {
        self.run_loop_with(a, v, u, Duration::from_millis(30))
    }

    pub fn run_loop_with<A, U, V>(
        &mut self,
        app: &mut A,
        mut view: V,
        mut update: U,
        timeout: Duration,
    ) -> Result<()>
    where
        V: FnMut(&mut A, &mut Frame),
        U: FnMut(&mut A, Event) -> Result<bool>,
    {
        loop {
            self.terminal.draw(|frame| view(app, frame))?;
            if poll(timeout)? && update(app, read()?)? {
                break;
            }
        }
        Ok(())
    }

    fn uninit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

impl<B: Write> Drop for Ui<B> {
    fn drop(&mut self) {
        self.uninit()
            .expect("got error when drop UiManager, something bad happen!")
    }
}
