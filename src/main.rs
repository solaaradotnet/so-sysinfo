use anyhow::Result;
use clap::Parser;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
        },
        ExecutableCommand,
    },
    Terminal,
};
use std::io::stdout;

mod app;
mod args;
mod collection;
mod logos;

pub fn init_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn init_tui() -> std::io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(SetTitle("so-sysinfo"))?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()> {
    init_panic_hook();
    let args = args::Args::parse();

    let mut terminal = init_tui()?;
    terminal.clear()?;

    app::app(terminal, args)?;

    restore_tui()?;

    Ok(())
}
