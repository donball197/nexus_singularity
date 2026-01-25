use std::{io, time::Duration};
use std::sync::Arc;
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::*};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::*, Terminal};
use crate::gemma_engine::GemmaEngine;

pub async fn run_terminal(_ai: Arc<GemmaEngine>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut input_text = String::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(f.area());

            let main_block = Block::default().title(" Hive Mind ").borders(Borders::ALL);
            f.render_widget(Paragraph::new("Gemma 3 Online. Pulse: 528Hz").block(main_block), chunks[0]);

            let input_block = Block::default().title(" Forge Command ").borders(Borders::ALL);
            let input_para = Paragraph::new(format!("> {}", input_text)).block(input_block);
            f.render_widget(input_para, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input_text.push(c),
                    KeyCode::Backspace => { input_text.pop(); }
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        // Neural trigger logic goes here
                        input_text.clear();
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
