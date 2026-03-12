use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    Frame, Terminal,
};
use std::io;

const APP_NAME: &str = "Obsidian CLI Inspector";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_TAGLINE: &str = "Local-first CLI/TUI for indexing and querying Obsidian vaults";

/// ASCII logo for the application header
const APP_LOGO: &str = r#"
    __    __    __                  __         __    __                 __ 
   / /   / /   / /  ___            / /        / /   / /   ___   ____   / /_
  / /   / /   / /  / _ \          / /        / /   / /   / _ \ / __ \ / __ \
 / /___/ /___/ /  /  __/         / /____    / /___/ /___/  __// /_/ // /_/ /
/_____/_____/_/   \___/          /______/   /_____/_____/\___/ \____//_____/ 
"#;

/// Render the application header (title bar / hero banner)
fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    use ratatui::prelude::Stylize;

    let logo_lines: Vec<Line> = APP_LOGO
        .lines()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::Cyan).bold())))
        .collect();

    let title = Line::from(vec![
        Span::styled(APP_NAME, Style::default().fg(Color::LightCyan).bold()),
        Span::raw(" "),
        Span::styled(
            format!("v{APP_VERSION}"),
            Style::default().fg(Color::Yellow),
        ),
    ]);

    let tagline = Line::from(Span::styled(
        APP_TAGLINE,
        Style::default().fg(Color::DarkGray),
    ));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(logo_lines.len() as u16),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    // Render logo
    for (i, line) in logo_lines.iter().enumerate() {
        if i < chunks[0].height as usize {
            let x = chunks[0].x + (chunks[0].width.saturating_sub(line.width() as u16)) / 2;
            frame.render_widget(
                Line::from(line.spans.clone()),
                ratatui::layout::Rect {
                    x,
                    y: chunks[0].y + i as u16,
                    width: line.width() as u16,
                    height: 1,
                },
            );
        }
    }

    // Render title and version
    frame.render_widget(title, chunks[1]);

    // Render tagline
    frame.render_widget(tagline, chunks[2]);
}

/// Main TUI run loop
pub fn run_tui() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Run the main loop - for now just render the header and exit on 'q'
    loop {
        terminal.draw(|f| {
            let area = f.area();

            // Create main layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length((APP_LOGO.lines().count() + 3) as u16), // Header
                    Constraint::Min(0),                                        // Content
                ])
                .split(area);

            // Render header
            render_header(f, chunks[0]);

            // Placeholder for future content
            use ratatui::prelude::Stylize;
            let placeholder = Line::from(Span::styled(
                "Press 'q' to quit...",
                Style::default().fg(Color::DarkGray).italic(),
            ));
            f.render_widget(
                placeholder,
                ratatui::layout::Rect {
                    x: area.x,
                    y: area.y.saturating_sub(1),
                    width: area.width,
                    height: 1,
                },
            );
        })?;

        // Check for quit
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(crossterm::event::KeyEvent {
                code: crossterm::event::KeyCode::Char('q'),
                ..
            }) = crossterm::event::read()?
            {
                break;
            }
        }
    }

    Ok(())
}

pub fn show_tui(logger: Option<&crate::logger::Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("tui", "Starting TUI");
    }

    // Try to run the TUI
    match run_tui() {
        Ok(_) => {}
        Err(e) => {
            if let Some(log) = logger {
                let _ = log.print_and_log("tui", &format!("TUI error: {e}"));
            } else {
                eprintln!("TUI error: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_app_constants_defined() {
        assert!(!APP_NAME.is_empty());
        assert!(!APP_VERSION.is_empty());
        assert!(!APP_TAGLINE.is_empty());
        assert!(!APP_LOGO.is_empty());
    }

    #[test]
    fn test_version_format() {
        // Version should be in format x.y.z
        let parts: Vec<&str> = APP_VERSION.split('.').collect();
        assert!(parts.len() >= 2);
    }
}
