use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

mod timer;
use timer::{Timer, TimerEvent};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    timer: Timer,
}

impl App {
    // This is the lifetime manager for the application
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.timer.update();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('w') => self.timer.handle_input(TimerEvent::StartWork),
            KeyCode::Char('b') => self.timer.handle_input(TimerEvent::StartBreak),
            KeyCode::Char('p') => self.timer.handle_input(TimerEvent::Pause),
            KeyCode::Char('r') => self.timer.handle_input(TimerEvent::Resume),
            KeyCode::Char('x') => self.timer.handle_input(TimerEvent::Reset),
            _ => {}
        }
    }
    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{:02}:{:02}", minutes, seconds)
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tomate ".red().bold());
        let instructions = Line::from(vec![
            " Start Work: ".into(),
            "<w>".blue().bold(),
            " Start Break: ".into(),
            "<b>".blue().bold(),
            " Pause: ".into(),
            "<p>".blue().bold(),
            " Resume: ".into(),
            "<r>".blue().bold(),
            " Reset: ".into(),
            "<x>".blue().bold(),
            " Quit: ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let remaining_time = self.timer.get_remaining_time();
        let counter_text = Text::from(vec![Line::from(vec![
            "Remaining: ".into(),
            App::format_duration(remaining_time).into(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
