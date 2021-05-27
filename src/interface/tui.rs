use std::
{
    error::Error,
    io::stdout,
    sync::mpsc,
    threadm
    time::{duration, Instant},
};
use tui::{Terminal, backend::CrosstermBackend};
use crossterm::
{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode}.
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn launch_tui() -> Result<(), io:Error>
{
    let cli: Cli = argh::fromenv();

    enable_raw_mode();

    let stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    Ok(())
}