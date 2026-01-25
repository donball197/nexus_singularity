use std::{io, time::Duration, sync::Arc};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::*};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::*, Terminal};
use crate::gemma_engine::GemmaEngine;

pub async fn run_terminal(ai: Arc<GemmaEngine>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut input_text = String::new();
    let mut neural_feed = String::from("Nexus v0.2.2 Online | API Mode");

    loop {
        terminal.draw(|f| {
            let area = f.size(); 
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(area);

            f.render_widget(Paragraph::new(neural_feed.as_str()).block(Block::default().title(" Hive Mind ").borders(Borders::ALL)), chunks[0]);
            f.render_widget(Paragraph::new(format!("> {}", input_text)).block(Block::default().title(" Forge Command ").borders(Borders::ALL)), chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input_text.push(c),
                    KeyCode::Backspace => { input_text.pop(); }
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        let cmd = input_text.clone();
                        input_text.clear();
                        if let Ok(res) = ai.process_thought(&cmd).await {
                            neural_feed = res;
                        }
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
