use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

mod timer;
mod visuals;
use timer::{Timer, TimerEvent};

use crate::visuals::draw_time;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    timer: Timer,
}

impl App {
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
    fn render_duration_as_lines<'a>(duration: Duration) -> Vec<Line<'a>> {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        let minute_tens = minutes / 10;
        let minute_ones = minutes % 10;
        let second_tens = seconds / 10;
        let second_ones = seconds % 10;

        let time_lines = draw_time(minute_tens, minute_ones, second_tens, second_ones);
        time_lines
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn relevant_instructions<'a>(&self) -> Line<'a> {
        match self.timer.state() {
            timer::TimerState::Idle => Line::from(vec![
                " Start Work:".into(),
                "<w>".blue().bold(),
                " Start Break:".into(),
                "<b>".blue().bold(),
                " Exit:".into(),
                "<q>".blue().bold(),
            ]),
            timer::TimerState::Running => Line::from(vec![
                " Pause:".into(),
                "<p>".blue().bold(),
                " Exit:".into(),
                "<q>".blue().bold(),
            ]),
            timer::TimerState::Paused => Line::from(vec![
                " Resume:".into(),
                "<r>".blue().bold(),
                " Reset:".into(),
                "<x>".blue().bold(),
                " Exit:".into(),
                "<q>".blue().bold(),
            ]),
            timer::TimerState::Completed => Line::from(vec![
                " Reset:".into(),
                "<x>".blue().bold(),
                " Exit:".into(),
                "<q>".blue().bold(),
            ]),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tomate ".red().bold());
        let instructions = self.relevant_instructions();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let remaining_time = self.timer.get_remaining_time();

        let time = App::render_duration_as_lines(remaining_time);

        Paragraph::new(time)
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
