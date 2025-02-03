mod models;
mod ui;
mod storage;

use crossterm::{terminal, execute};
use tui::{backend::CrosstermBackend, Terminal};
use std::{io, error::Error};
use models::AppState;
use storage::{save_budget, load_budget};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state = AppState::new();

    // Load existing budget
    if let Ok(budget) = load_budget() {
        state.budget = budget;
    }

    // Main loop
    loop {
        terminal.draw(|f| ui::render(f, &state))?;
        
        if !ui::handle_input(&mut state) {
            break;
        }
    }

    // Save and cleanup
    save_budget(&state.budget)?;
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}
