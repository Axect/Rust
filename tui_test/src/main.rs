use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Block
    //terminal.draw(|mut f| {
    //    let size = f.size();
    //    Block::default()
    //        .title("Block")
    //        .borders(Borders::ALL)
    //        .render(&mut f, size);
    //})
    
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
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, chunks[0]);
        Block::default()
            .title("Block 1")
            .borders(Borders::ALL)
            .render(&mut f, chunks[1]);
        Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
            .render(&mut f, chunks[2]);
    })
}
