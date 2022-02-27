use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("standard io errors")]
    IOError(#[from] io::Error),
    #[error("crossterm error kind")]
    CrossTermError(#[from] crossterm::ErrorKind),
}

fn main() -> Result<(), TerminalError> {
    // enable terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Testing").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    //
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
