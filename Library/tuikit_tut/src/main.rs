extern crate tuikit;

use tuikit::prelude::*;

fn main() -> Result<()> {
    win_tut()?;
    Ok(())
}

fn split_tut() -> Result<()> {
    let term = Term::with_height(TermHeight::Percent(100)).unwrap();
    let mean = Mean("안녕하세요".to_string());
    let word = Word("Hello".to_string());

    while let Ok(ev) = term.poll_event() {
        match ev {
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => break,
            _ => (),
        }

        let hsplit = HSplit::default()
            .split(
                VSplit::default()
                    .shrink(0)
                    .split(Win::new(&word).border(true))
                    .split(Win::new(&word).border(true)),
            )
            .split(Win::new(&mean).border(true));

        term.draw(&hsplit)?;
        term.present()?;
    }
    Ok(())
}

struct Word(String);

impl Draw for Word {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (_width, height) = canvas.size()?;
        let top = height / 2;
        let _ = canvas.print(top, 0, &self.0);
        Ok(())
    }
}

impl Widget for Word {
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (Some(self.0.len()), None)
    }
}

struct Mean(String);

impl Draw for Mean {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.0.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        let _ = canvas.print_with_attr(0, left, "press 'q' to exit", Effect::UNDERLINE.into());
        let _ = canvas.print(top, left, &self.0);
        Ok(())
    }
}

impl Widget for Mean {}

struct Model(String);

impl Draw for Model {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.0.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        canvas.print(top, left, &self.0)?;
        Ok(())
    }
}

impl Widget for Model {}

fn win_tut() -> Result<()> {
    let term = Term::with_height(TermHeight::Percent(100)).unwrap();
    let model = Model("Good!".to_string());
    let model2 = Model("Bad!".to_string());

    while let Ok(ev) = term.poll_event() {
        match ev {
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => break,
            _ => (),
        }
        term.print(0, 0, "press 'q' to exit")?;
        let inner_win = Win::new(&model).border(true);
        let inner_split = HSplit::default()
            .shrink(10)
            .split(inner_win)
            .split(Win::new(&model2).border(true));

        let win = Win::new(&inner_split)
            .margin(Size::Percent(10))
            .padding(1)
            .border(true)
            .border_top_attr(Color::BLUE)
            .border_right_attr(Color::YELLOW)
            .border_bottom_attr(Color::RED)
            .border_left_attr(Color::GREEN);

        term.draw(&win)?;
        term.present()?;
    }

    Ok(())
}
