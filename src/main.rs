mod app;
mod storage;

use crossterm::{execute, terminal};
use std::{io, error::Error};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = app::App::new();
    
    // Load existing data
    app.transactions = storage::load_transactions().unwrap_or_default();
    
    // Main loop
    loop {
        terminal.draw(|f| app.render(f))?;
        if !app.handle_input()? {
            break;
        }
    }
    
    // Save data and cleanup
    storage::save_transactions(&app.transactions)?;
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}
