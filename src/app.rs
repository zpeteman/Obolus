use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color, Modifier},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, BarChart},
    Frame,
};
use crossterm::event::{self, Event, KeyCode};
use chrono::Local;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub amount: f64,
    pub category: String,
    pub date: String,
    pub description: String,
}

pub struct App {
    pub transactions: Vec<Transaction>,
    selected_transaction: Option<usize>,
    input_field: usize,
    input_amount: String,
    input_category: String,
    input_date: String,
    input_description: String,
    focus_mode: FocusMode,
}

#[derive(PartialEq)]
enum FocusMode {
    Transactions,
    Input,
}

impl App {
    pub fn new() -> Self {
        App {
            transactions: Vec::new(),
            selected_transaction: None,
            input_field: 0,
            input_amount: String::new(),
            input_category: String::new(),
            input_date: String::new(),
            input_description: String::new(),
            focus_mode: FocusMode::Transactions,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(10), // Charts
                Constraint::Min(8),     // Transactions
                Constraint::Length(10), // Input (increased height)
            ])
            .split(f.size());

        self.render_header(f, chunks[0]);
        self.render_charts(f, chunks[1]);
        self.render_transactions(f, chunks[2]);
        self.render_input(f, chunks[3]);
    }

    fn render_header<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let total: f64 = self.transactions.iter().map(|t| t.amount).sum();
        let balance = Spans::from(vec![
            Span::styled("Total: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.2}", total),
                Style::default()
                    .fg(if total >= 0.0 { Color::Green } else { Color::Red })
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        f.render_widget(Paragraph::new(balance), area);
    }
 
    fn render_charts<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
    
        // Monthly spending
        let monthly_data = self.transactions.iter()
            .fold(HashMap::new(), |mut acc, t| {
                if let Some(month) = t.date.split('-').nth(1) {
                    *acc.entry(month).or_insert(0.0) += t.amount.abs();
                }
                acc  // Explicitly return accumulator
            })
            .into_iter()
            .map(|(m, v)| (m, v as u64))
            .collect::<Vec<_>>();
    
        let monthly = BarChart::default()
            .block(Block::default().title("Monthly Spending"))
            .data(&monthly_data)
            .bar_width(6)
            .bar_style(Style::default().fg(Color::Yellow));
    
        // Category breakdown (percentage)
        let category_totals = self.transactions.iter()
            .filter(|t| t.amount < 0.0)
            .fold(HashMap::new(), |mut acc, t| {
                *acc.entry(t.category.as_str()).or_insert(0.0) += t.amount.abs();
                acc  // Explicitly return accumulator
            });
    
        let total_expenses: f64 = category_totals.values().sum();
    
        let category_data = category_totals
            .into_iter()
            .map(|(c, v)| {
                let percentage = if total_expenses == 0.0 {
                    0.0
                } else {
                    (v / total_expenses) * 100.0
                };
                (c, percentage.round() as u64)
            })
            .collect::<Vec<_>>();
    
        let categories = BarChart::default()
            .block(Block::default().title("Categories (% of Spending)"))
            .data(&category_data)
            .bar_width(12)
            .bar_style(Style::default().fg(Color::Magenta));
    
        f.render_widget(monthly, chunks[0]);
        f.render_widget(categories, chunks[1]);
    } 

    fn render_transactions<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<Spans> = self.transactions
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let selected = self.selected_transaction == Some(i);
                Spans::from(vec![
                    Span::styled(if selected { "▶ " } else { "  " }, Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("{:8.2} ", t.amount),
                        Style::default().fg(if t.amount < 0.0 { Color::Red } else { Color::Green })
                    ),
                    Span::styled(format!("{:12} ", t.category), Style::default().fg(Color::Cyan)),
                    Span::raw(format!("{} - {}", t.date, t.description)),
                ])
            })
            .collect();

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Transactions (↑/↓/d)");
        
        f.render_widget(Paragraph::new(items).block(block), area);
    }

    fn render_input<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Title
                Constraint::Length(3),  // Input fields
                Constraint::Length(5),  // Description (more space)
            ])
            .split(area);

        // Title
        f.render_widget(
            Paragraph::new("Add Transaction (a to start, Tab to navigate)")
                .style(Style::default().fg(Color::Gray)),
            chunks[0]
        );

        // Fields
        let fields = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(chunks[1]);

        let inputs = [
            format!("Amount: {}", self.input_amount),
            format!("Category: {}", self.input_category),
            format!("Date: {}", self.input_date),
            format!("Desc: {}", self.input_description),
        ];

        for (i, (text, area)) in inputs.iter().zip(fields.iter()).enumerate() {
            let style = if i == self.input_field {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            f.render_widget(Paragraph::new(text.as_str()).style(style), *area);
        }

        // Description preview (with wrapping)
        f.render_widget(
            Paragraph::new(self.input_description.as_str())
                .wrap(tui::widgets::Wrap { trim: true })
                .block(Block::default().borders(Borders::TOP)),
            chunks[2]
        );
    }

    pub fn handle_input(&mut self) -> Result<bool, std::io::Error> {
        if let Event::Key(key) = event::read()? {
            match self.focus_mode {
                FocusMode::Transactions => match key.code {
                    KeyCode::Esc => return Ok(false),
                    KeyCode::Char('a') => {
                        self.focus_mode = FocusMode::Input;
                        self.input_field = 0;
                    }
                    KeyCode::Down => self.move_selection(1),
                    KeyCode::Up => self.move_selection(-1),
                    KeyCode::Char('d') => self.delete_selected(),
                    _ => (),
                },
                FocusMode::Input => match key.code {
                    KeyCode::Esc => {
                        self.clear_inputs();
                        self.focus_mode = FocusMode::Transactions;
                    }
                    KeyCode::Enter => {
                        self.save_transaction();
                        self.focus_mode = FocusMode::Transactions;
                    }
                    KeyCode::Tab => self.input_field = (self.input_field + 1) % 4,
                    KeyCode::BackTab => self.input_field = (self.input_field + 3) % 4,
                    KeyCode::Backspace => {
                        match self.input_field {
                            0 => { self.input_amount.pop(); }
                            1 => { self.input_category.pop(); }
                            2 => { self.input_date.pop(); }
                            3 => { self.input_description.pop(); }
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => match self.input_field {
                        0 => self.input_amount.push(c),
                        1 => self.input_category.push(c),
                        2 => self.input_date.push(c),
                        3 => self.input_description.push(c),
                        _ => (),
                    },
                    _ => (),
                },
            }
        }
        Ok(true)
    }

    // Helper methods
    fn move_selection(&mut self, delta: i32) {
        let new_index = match self.selected_transaction {
            Some(i) => (i as i32 + delta).max(0) as usize,
            None if !self.transactions.is_empty() => 0,
            _ => return,
        };
        if new_index < self.transactions.len() {
            self.selected_transaction = Some(new_index);
        }
    }

    fn delete_selected(&mut self) {
        if let Some(index) = self.selected_transaction {
            if index < self.transactions.len() {
                self.transactions.remove(index);
                self.selected_transaction = if self.transactions.is_empty() {
                    None
                } else {
                    Some(index.saturating_sub(1))
                };
            }
        }
    }

    fn save_transaction(&mut self) {
        let amount = self.input_amount.parse().unwrap_or(0.0);
        let date = if self.input_date.is_empty() {
            Local::now().format("%Y-%m-%d").to_string()
        } else {
            self.input_date.clone()
        };

        self.transactions.push(Transaction {
            amount,
            category: self.input_category.clone(),
            date,
            description: self.input_description.clone(),
        });

        self.clear_inputs();
    }

    fn clear_inputs(&mut self) {
        self.input_amount.clear();
        self.input_category.clear();
        self.input_date.clear();
        self.input_description.clear();
        self.input_field = 0;
    }
}

