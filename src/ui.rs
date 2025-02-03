use std::io;
use tui::backend::{CrosstermBackend};
use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use tui::Terminal;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{self, ClearType};
use crossterm::cursor;
use crossterm::ExecutableCommand;

struct Transaction {
    amount: f64,
    category: String,
    description: String,
    date: String,
}

struct Budget {
    transactions: Vec<Transaction>,
}

struct AppState {
    budget: Budget,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            budget: Budget {
                transactions: vec![],
            },
        }
    }

    fn add_transaction(&mut self, amount: f64, category: String, description: String, date: String) {
        let transaction = Transaction {
            amount,
            category,
            description,
            date,
        };
        self.budget.transactions.push(transaction);
    }

    fn update_budget(&mut self) {
        let income = self.budget.transactions.iter().filter(|t| t.amount > 0.0).map(|t| t.amount).sum::<f64>();
        let expenses = self.budget.transactions.iter().filter(|t| t.amount < 0.0).map(|t| t.amount).sum::<f64>();
        let balance = income + expenses; // Using sum to calculate balance
        println!("Balance: {:.2}", balance);
    }
}

fn render(f: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let info = format!("Budget Transactions: \n\n");

    f.render_widget(Paragraph::new(info).style(Style::default().fg(Color::White)), chunks[0]);

    f.render_widget(
        Block::default().borders(Borders::ALL).title("Transaction List"),
        chunks[1],
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = AppState::new();

    app_state.add_transaction(200.0, "Salary".to_string(), "Monthly salary".to_string(), "2025-02-01".to_string());
    app_state.add_transaction(-50.0, "Groceries".to_string(), "Weekly groceries".to_string(), "2025-02-02".to_string());
    app_state.update_budget();

    terminal::enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.clear(ClearType::All)?;
        render(&mut terminal, &app_state);
        terminal.flush()?;

        if event::poll(std::time::Duration::from_millis(1000))? {
            if let event::Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            }) = event::read()?
            {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
