use std::{error::Error, io};
use termion::{
    raw::IntoRawMode,
    input::TermRead,
};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction},
    widgets::{Widget, Block, Borders, Paragraph, Text},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // Create Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Building UI
    //terminal.draw(|mut f| {
    //    let size = f.size();
    //    let block = Block::default()
    //        .title("Block")
    //        .borders(Borders::ALL);
    //    f.render_widget(block, size);
    //})
    
    // Layout
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
}
